use crate::try_from_chunks::TryFromChunks;
use crate::{FieldMetadata, TypedMultipartError};
use axum::async_trait;
use axum::extract::multipart::Field;
use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;
use std::mem;

/// Types that can be created from an instance of [Field].
///
/// All fields for a given struct must implement this trait to be able to derive
/// the [TryFromMultipart](crate::TryFromMultipart) trait.
///
/// Implementing this trait directly is not recommended since it requires the
/// user to manually implement the size limit logic. Instead, implement the
/// [TryFromChunks] trait and this trait will be implemented automatically.
#[async_trait]
pub trait TryFromField: Sized {
    /// Consume the input [Field] to create the supplied type.
    ///
    /// The `limit_bytes` parameter is used to limit the size of the field. If
    /// the field is larger than the limit, an error is returned.
    async fn try_from_field(
        field: Field<'_>,
        limit_bytes: Option<usize>,
    ) -> Result<Self, TypedMultipartError>;
}

#[async_trait]
impl<T> TryFromField for T
where
    T: TryFromChunks + Send + Sync,
{
    async fn try_from_field(
        field: Field<'_>,
        limit_bytes: Option<usize>,
    ) -> Result<Self, TypedMultipartError> {
        let metadata = FieldMetadata::from(&field);
        let mut field_name = metadata.name.clone().unwrap_or(String::new());
        let mut size_bytes = 0;

        let chunks = field.map_err(TypedMultipartError::from).map(|chunk| {
            if let Ok(chunk) = chunk.as_ref() {
                size_bytes += chunk.len();

                if let Some(limit_bytes) = limit_bytes {
                    if size_bytes > limit_bytes {
                        return Err(TypedMultipartError::FieldTooLarge {
                            field_name: mem::take(&mut field_name),
                            limit_bytes,
                        });
                    }
                }
            }

            chunk
        });

        T::try_from_chunks(chunks, metadata).await
    }
}

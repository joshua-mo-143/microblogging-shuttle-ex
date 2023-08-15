use anyhow::anyhow;
use axum::body::Bytes;
use s3::Bucket;

pub async fn upload_image(
    data: Bytes,
    filename: String,
    bucket: Bucket,
) -> Result<(), anyhow::Error> {
    let res_data = bucket.put_object(&filename, &data).await.unwrap();

    match res_data.status_code() {
        200u16 => Ok(()),
        _ => Err(anyhow!(
            "Something went wrong! Please contact the owner for assistance."
        )),
    }
}

pub async fn delete_image(key: String, bucket: Bucket) -> Result<(), anyhow::Error> {
    let res_data = bucket.delete_object(&key).await.unwrap();

    match res_data.status_code() {
        200u16 => Ok(()),
        _ => Err(anyhow!(
            "Something went wrong! Please contact the owner for assistance."
        )),
    }
}

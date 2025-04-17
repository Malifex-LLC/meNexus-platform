const { PutObjectCommand } = require("@aws-sdk/client-s3");
const s3 = require("../config/s3");
const fs = require("fs");
const mime = require('mime-types');

exports.uploadToS3 = async (localPath, key) => {
    const fileStream = fs.createReadStream(localPath);
    const contentType = mime.lookup(key) || 'application/octet-stream';

    const command = new PutObjectCommand({
        Bucket: process.env.AWS_S3_BUCKET_NAME,
        Key: key,
        Body: fileStream,
        ContentType: contentType,
        }
    );

    await s3.send(command);

    return `https://${process.env.AWS_S3_BUCKET_NAME}.s3.${process.env.AWS_REGION}.amazonaws.com/${key}`;
}


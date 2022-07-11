use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PngError {
    #[error("invalid chunkType param error")]
    ChunkTypeError,

    #[error("invalid chunk error")]
    ChunkError,

    #[error("CRC error")]
    CRCError,

    #[error("Chunk header error")]
    ChunkHeaderError,

    #[error("there is not such chunkType")]
    NotFoundChunkType,

    #[error("IO error")]
    IOError(#[from] io::Error),
}

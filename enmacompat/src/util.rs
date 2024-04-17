use std::io::{Read, Seek, SeekFrom};

pub fn find_in_data<T: Read + Seek>(
    data: &mut T,
    target: &[u8],
) -> Result<Vec<usize>, std::io::Error> {
    let pos = data.stream_position()?;
    let mut result = Vec::new();
    let mut possible = vec![false; target.len()];
    let mut possible_i = 0;

    for (i, byte) in data.bytes().enumerate() {
        let byte = byte?;
        if possible[possible_i] {
            result.push(i - target.len());
        }
        possible[possible_i] = byte == target[possible_i];

        for j in 0..possible.len() {
            let target_index = (possible_i + j) % target.len();

            possible[j] = possible[j] && byte == target[target_index];
        }

        possible_i = (possible_i + 1) % target.len();
    }

    data.seek(SeekFrom::Start(pos))?;

    Ok(result)
}

pub fn is_exactly<T: Read + Seek>(
    data: &mut T,
    location: usize,
    target: &[u8],
) -> Result<bool, std::io::Error> {
    let pos = data.stream_position()?;
    data.seek(SeekFrom::Start(location as u64))?;
    let mut buf = [0; 8];
    let result = data.read_exact(&mut buf);
    data.seek(SeekFrom::Start(pos))?;
    Ok(result.is_ok() && buf == target)
}

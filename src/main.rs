#![allow(dead_code)]
mod init;
mod cli;
mod cat_file;
mod hash_object;
mod repo;
mod test;
mod objects;
mod write_tree;

fn main() {
    cli::parse()
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    #[test]
    fn test1() {
        
        let bytes1 = &b"Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks.Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks.Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks.Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks."[..];
        let bytes2 = &b"Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks.Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks.Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks.Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks.Creating a new Vec from multiple iterators is typically slower than concatenating vectors directly because iterators involve traversal logic and potentially heap allocations for each element, whereas concatenating vectors can directly copy memory blocks."[..];
    
        let start = Instant::now();
        let _combined_vec = bytes1.iter().chain(bytes2.iter()).cloned().collect::<Vec<_>>();
        let elapsed = start.elapsed();
        println!("Time taken to concatenate iterators: {:?}", elapsed);

        let start = Instant::now();
        let mut vec1 = bytes1.to_vec();
        let _combined_vec = vec1.extend_from_slice(bytes2);
        let elapsed = start.elapsed();
        println!("Time taken to concatenate iterators: {:?}", elapsed);

        let start = Instant::now();
        let mut vec1 = bytes1.to_vec();
        vec1.splice(0..0, bytes2.iter().cloned());
        let elapsed = start.elapsed();
        println!("Time taken to concatenate iterators: {:?}", elapsed);
    
        let start = Instant::now();
        let _concatenated_vec = [&bytes1[..], &bytes2[..]].concat();
        let elapsed = start.elapsed();
        println!("Time taken to concatenate vectors: {:?}", elapsed);

        let start = Instant::now();
        let _c = 6u64 + 30u64;
        let v: Vec<u8> = vec![1,2];
        let elapsed = start.elapsed();
        println!("Time taken to add two u64: {:?},{:?}", elapsed, v.capacity());
        
    }
}
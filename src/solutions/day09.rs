use std::{cmp::{min, Reverse}, collections::BinaryHeap, usize};
use crate::common::read_file;

pub fn run(file_path: &str) -> (usize, usize) {
    let disk_summary: Vec<usize> = read_file(file_path).trim().chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    (part1(disk_summary.clone()), part2(disk_summary.clone()))
}

fn part1(mut disk_summary: Vec<usize>) -> usize {
    let (mut i_left, mut i_right) = (0 as usize, disk_summary.len() - 2 + disk_summary.len()%2); // last file

    let mut disk = DiskMeta::new();
    while i_left < i_right {
        if i_left % 2 == 0 {
            disk.write_file(i_left/2, disk_summary[i_left]);
            i_left += 1;
        } else {
            let move_amount = min(disk_summary[i_left], disk_summary[i_right]);

            disk.write_file(i_right/2, if i_left == i_right - 1 {disk_summary[i_right] } else {move_amount});
            disk_summary[i_left] -= move_amount;
            disk_summary[i_right] -= move_amount;
            if disk_summary[i_left] == 0 {
                i_left += 1;
            }
            if disk_summary[i_right] == 0 {
                i_right -= 2;
            }
        }
    }
    return disk.check_sum;
}

fn part2(disk_summary: Vec<usize>) -> usize {
    let mut files: Vec<(usize, FileMeta)> = Vec::new(); // (original_offset, filemeta)
    let mut free_spaces = DiskSegments::new();
    let mut current_offset = 0;
    for (i, &size) in disk_summary.iter().enumerate() {
        if i % 2 == 0 { // file
            files.push((current_offset, FileMeta{id: i/2, size}));
        } else { // free space
            free_spaces.add_segment(current_offset, size);
        }
        current_offset += size;
    }

    let mut checksum = 0;
    for (original_offset, file_meta) in files.iter().rev() {
        let final_offset = match free_spaces.use_segment(*original_offset, file_meta.size) {
            Some(moved_offset) => moved_offset,
            None => *original_offset
        };
        checksum += calc_checksum(file_meta, final_offset);
    }
    return checksum;
}

fn calc_checksum(file: &FileMeta, from_offset: usize) -> usize{
    // println!("File {} at {from_offset}", file.id);
    let mut res = 0;
    for i in 0..file.size {
        res += (from_offset + i) * file.id;
    }
    return res;
}


struct DiskMeta { i_disk: usize, check_sum: usize }
impl DiskMeta {
    fn new() -> DiskMeta { DiskMeta{i_disk: 0, check_sum: 0} }
    fn write_file(&mut self, file_id: usize, size: usize) {
        for _ in 0..size {
            self.check_sum += self.i_disk * file_id;
            self.i_disk += 1;
        }
    }
}

struct FileMeta {id: usize, size: usize }
struct DiskSegments { free_segments: Vec<BinaryHeap<Reverse<usize>>> }
impl DiskSegments {
    fn new() -> DiskSegments { DiskSegments{free_segments: (0..10).into_iter().map(|_| BinaryHeap::new()).collect()} }

    fn add_segment(&mut self, offset: usize, size: usize) {
        self.free_segments[size].push(Reverse(offset));
    }

    // start offset of segment, if a good segment is found
    fn use_segment(&mut self, file_offset: usize, file_size: usize) -> Option<usize> {
        let (mut best_segment_offset, mut best_segment_size) = (usize::MAX, 0usize);

        for size in file_size..self.free_segments.len() {
            let top = self.free_segments[size].peek();
            if top.is_none() {
                continue;
            }
            let offset = top.unwrap().0;
            if offset < best_segment_offset {
                best_segment_offset = offset;
                best_segment_size = size;
            }
        }

        // No free space of larger size found to the left of file  
        if file_offset < best_segment_offset { 
            return None;
        }

        self.free_segments[best_segment_size].pop();
        let remaining_free_size = best_segment_size - file_size;
        if remaining_free_size > 0 {
            let remaining_start_offset = best_segment_offset + file_size;
            self.free_segments[remaining_free_size].push(Reverse(remaining_start_offset));
        }
        return Some(best_segment_offset)
    }
}
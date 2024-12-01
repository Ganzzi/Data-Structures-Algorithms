use stack::stack::Stack;

/// # Tower of Hanoi Puzzle.
///
/// This function solves the Tower of Hanoi puzzle recursively.
///
/// # Arguments
///
/// * `tower_height` - The number of disks to be moved.
/// * `src_peg` - The source peg (stack) from which disks are moved.
/// * `mid_peg` - The auxiliary peg (stack) used for temporary storage.
/// * `des_peg` - The destination peg (stack) to which disks are moved.
///
/// # Examples
///
/// ```
/// use crate::{recursion::tower_of_hanoi::tower_of_hanoi};
/// use stack::stack::Stack;
///
/// let mut src_peg = Stack::new();
/// src_peg.push(1);
/// src_peg.push(2);
/// src_peg.push(3);
/// let mut mid_peg = Stack::new();
/// let mut des_peg = Stack::new();
/// tower_of_hanoi(src_peg.size() as u32, &mut src_peg, &mut mid_peg, &mut des_peg);
///
/// assert_eq!(src_peg.size(), 0);
/// assert_eq!(mid_peg.size(), 0);
/// assert_eq!(des_peg.size(), 3);
/// ```
pub fn tower_of_hanoi(
    tower_height: u32,
    src_peg: &mut Stack<u32>,
    mid_peg: &mut Stack<u32>,
    des_peg: &mut Stack<u32>,
) {
    if 0 == tower_height {
        return;
    }

    tower_of_hanoi(tower_height - 1, src_peg, des_peg, mid_peg);

    des_peg.push(src_peg.pop().unwrap());

    tower_of_hanoi(tower_height - 1, mid_peg, src_peg, des_peg);
}

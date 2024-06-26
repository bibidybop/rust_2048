// #[cfg(test)]
// mod tests {
//     use board_manipulations::*;

//     #[test]
//     fn test_left() {
//         let b = [2, 2, 4, 4, 2, 4, 4, 4, 2, 4, 8, 4, 2, 4, 8, 8];
//         let e: [i32; 16] = [4, 8, 0, 0, 2, 8, 4, 0, 2, 4, 8, 4, 2, 4, 16, 0];
//         assert_eq!(left(&b), e);
//         let b = [0, 2, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//         let e = [2, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//         assert_eq!(left(&b), e);
//         let tiles = [0, 2, 2, 0, 0, 0, 2, 2, 4, 0, 0, 4, 2, 0, 2, 4];
//         let expected: [i32; 16] = [4, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 0, 4, 4, 0, 0];
//         assert_eq!(left(&tiles), expected);
//         let tiles = [4, 4, 2, 2, 4, 4, 4, 2, 2, 4, 8, 4, 8, 8, 4, 2];
//         let expected: [i32; 16] = [8, 4, 0, 0, 8, 4, 2, 0, 2, 4, 8, 4, 16, 4, 2, 0];
//         assert_eq!(left(&tiles), expected);
//     }

//     #[test]
//     fn test_right() {
//         let b = [2, 2, 4, 4, 2, 4, 4, 4, 2, 4, 8, 4, 2, 4, 8, 8];
//         let e: [i32; 16] = [0, 0, 4, 8, 0, 2, 4, 8, 2, 4, 8, 4, 0, 2, 4, 16];
//         assert_eq!(right(&b), e);
//         let b = [0, 2, 2, 0, 0, 0, 2, 2, 4, 0, 0, 4, 2, 0, 2, 4];
//         let e: [i32; 16] = [0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 4, 4];
//         assert_eq!(right(&b), e);
//         let b = [0, 2, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//         let e = [0, 0, 2, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//         assert_eq!(right(&b), e);
//         let b = [0, 2, 2, 0, 0, 0, 2, 2, 4, 0, 0, 4, 2, 0, 2, 4];
//         let e: [i32; 16] = [0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 8, 0, 0, 4, 4];
//         assert_eq!(right(&b), e);
//         let b = [4, 4, 2, 2, 4, 4, 4, 2, 2, 4, 8, 4, 8, 8, 4, 2];
//         let e: [i32; 16] = [0, 0, 8, 4, 0, 4, 8, 2, 2, 4, 8, 4, 0, 16, 4, 2];
//         assert_eq!(right(&b), e);
//     }

//     #[test]
//     fn test_up() {
//         let b = [2, 2, 2, 2, 2, 4, 4, 4, 4, 4, 8, 8, 4, 8, 8, 4];
//         let e: [i32; 16] = [4, 2, 2, 2, 8, 8, 4, 4, 0, 8, 16, 8, 0, 0, 0, 4];
//         assert_eq!(up(&b), e);
//     }

//     #[test]
//     fn test_down() {
//         let tiles = [4, 8, 8, 4, 4, 4, 8, 8, 2, 4, 4, 4, 2, 2, 2, 2];
//         let expected: [i32; 16] = [0, 0, 0, 4, 0, 8, 16, 8, 8, 8, 4, 4, 4, 2, 2, 2];
//         assert_eq!(down(&tiles), expected);
//     }

//     #[test]
//     fn test_can_move_left() {
//         assert!(can_move_left(&[
//             2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//         assert!(!can_move_left(&[
//             2, 4, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//         assert!(!can_move_left(&[
//             2, 4, 8, 4, 2, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//     }

//     #[test]
//     fn test_can_move_right() {
//         assert!(!can_move_right(&[
//             2, 4, 8, 16, 0, 2, 4, 8, 2, 4, 2, 4, 0, 0, 0, 0
//         ]));
//         assert!(can_move_right(&[
//             2, 4, 8, 0, 16, 2, 4, 8, 2, 4, 2, 4, 0, 0, 0, 0
//         ]));
//         assert!(can_move_right(&[
//             2, 4, 8, 16, 0, 2, 4, 8, 2, 4, 2, 4, 0, 2, 0, 0
//         ]));
//     }

//     #[test]
//     fn test_can_move_up() {
//         assert!(can_move_left(&[
//             2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//         assert!(!can_move_left(&[
//             2, 4, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//         assert!(!can_move_left(&[
//             2, 4, 8, 4, 2, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//     }

//     #[test]
//     fn test_can_move_down() {
//         assert!(can_move_left(&[
//             2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//         assert!(!can_move_left(&[
//             2, 4, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//         assert!(!can_move_left(&[
//             2, 4, 8, 4, 2, 4, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0
//         ]));
//     }
// }

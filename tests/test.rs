use arr_ty::prelude::*;
use std::any::Any;

#[test]
fn test_num() {
    let arr = arr_ty!(u32;);
    assert_eq!(arr.len(), 0);

    let arr = arr_ty!(u32;,);
    assert_eq!(arr.len(), 0);

    let arr = arr_ty!(u32; 0; 3);
    assert_eq!(arr.len(), 3);
    assert!(arr.iter().all(|x| *x == 0));

    let arr = arr_ty!(u32; [0, 1, 2]);
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0], 0);
    assert_eq!(arr[1], 1);
    assert_eq!(arr[2], 2);

    let arr = arr_ty!(u32; [0, 1, 2,]);
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0], 0);
    assert_eq!(arr[1], 1);
    assert_eq!(arr[2], 2);

    let arr = arr_ty!((i32, u32); [(0, 0), (0, 1)]);
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0].0, 0);
    assert_eq!(arr[1].0, 0);
    assert_eq!(arr[0].1, 0u32);
    assert_eq!(arr[1].1, 1u32);
}

#[test]
fn test_dyn() {
    let arr = arr_ty!(Box<dyn Any>;);
    assert_eq!(arr.len(), 0);

    let arr = arr_ty!(Box<dyn Any>;,);
    assert_eq!(arr.len(), 0);

    let arr = arr_ty!(i32; 0; 3);
    assert_eq!(arr.len(), 3);
    assert!(arr.iter().all(|x| *x == 0));

    let arr = arr_ty!(Box<dyn Any>; [Box::new(0), Box::new(false), Box::new("false")]);
    assert_eq!(arr.len(), 3);
    assert_eq!(*arr[0].downcast_ref::<i32>().unwrap(), 0);
    assert_eq!(*arr[1].downcast_ref::<bool>().unwrap(), false);
    assert_eq!(*arr[2].downcast_ref::<&str>().unwrap(), "false");

    let arr = arr_ty!(Box<dyn Any>; [Box::new(0), Box::new(false), Box::new("false"),]);
    assert_eq!(arr.len(), 3);
    assert_eq!(*arr[0].downcast_ref::<i32>().unwrap(), 0);
    assert_eq!(*arr[1].downcast_ref::<bool>().unwrap(), false);
    assert_eq!(*arr[2].downcast_ref::<&str>().unwrap(), "false");

    let arr = arr_ty!((i32, Box<dyn Any>); [(0, Box::new(false)), (0, Box::new("false"))]);
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0].0, 0);
    assert_eq!(arr[1].0, 0);
    assert_eq!(*arr[0].1.downcast_ref::<bool>().unwrap(), false);
    assert_eq!(*arr[1].1.downcast_ref::<&str>().unwrap(), "false");
}

extern crate arr_ty;

use arr_ty::arr_ty;
use std::any::Any;

#[test]
fn test() {
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
}

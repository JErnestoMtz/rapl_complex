# Complex numbers for rapl

```Rust
    assert_eq!( 1 + 1.im(), C(1,1));
    assert_eq!( 1.im() + 1, C(1,1));
    assert_eq!( C(0., 2.) + C(2., 3.), C(2.,5.));
    //assert_approx_eq!(C::from_polar(1.0, PI/2.), C(0.,1.))
```
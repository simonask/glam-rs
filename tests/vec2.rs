#[macro_use]
mod support;

macro_rules! impl_vec2_tests {
    ($t:ty, $new:ident, $vec2:ident, $vec3:ident, $mask:ident) => {
        #[test]
        fn test_new() {
            let v = $new(1 as $t, 2 as $t);

            assert_eq!(v.x, 1 as $t);
            assert_eq!(v.y, 2 as $t);

            let t = (1 as $t, 2 as $t);
            let v = $vec2::from(t);
            assert_eq!(t, v.into());

            let a = [1 as $t, 2 as $t];
            let v = $vec2::from(a);
            let a1: [$t; 2] = v.into();
            assert_eq!(a, a1);

            let v = $vec2::new(t.0, t.1);
            assert_eq!(t, v.into());

            assert_eq!($vec2::new(1 as $t, 0 as $t), $vec2::unit_x());
            assert_eq!($vec2::new(0 as $t, 1 as $t), $vec2::unit_y());
        }

        #[test]
        fn test_fmt() {
            let a = $vec2::new(1 as $t, 2 as $t);
            assert_eq!(
                format!("{:?}", a),
                format!("{}({:?}, {:?})", stringify!($vec2), a.x, a.y)
            );
            // assert_eq!(format!("{:#?}", a), "$vec2(\n    1.0,\n    2.0\n)");
            assert_eq!(format!("{}", a), "[1, 2]");
        }

        #[test]
        fn test_zero() {
            let v = $vec2::zero();
            assert_eq!($new(0 as $t, 0 as $t), v);
            assert_eq!(v, $vec2::default());
        }

        #[test]
        fn test_splat() {
            let v = $vec2::splat(1 as $t);
            assert_eq!($vec2::one(), v);
        }

        #[test]
        fn test_accessors() {
            let mut a = $vec2::zero();
            a.x = 1 as $t;
            a.y = 2 as $t;
            assert_eq!(1 as $t, a.x);
            assert_eq!(2 as $t, a.y);
            assert_eq!($vec2::new(1 as $t, 2 as $t), a);

            let mut a = $vec2::zero();
            a[0] = 1 as $t;
            a[1] = 2 as $t;
            assert_eq!(1 as $t, a[0]);
            assert_eq!(2 as $t, a[1]);
            assert_eq!($vec2::new(1 as $t, 2 as $t), a);
        }

        #[test]
        fn test_dot_unsigned() {
            let x = $new(1 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
        }

        #[test]
        fn test_ops() {
            let a = $new(2 as $t, 4 as $t);
            assert_eq!($new(4 as $t, 8 as $t), (a + a));
            assert_eq!($new(0 as $t, 0 as $t), (a - a));
            assert_eq!($new(4 as $t, 16 as $t), (a * a));
            assert_eq!($new(4 as $t, 8 as $t), (a * 2 as $t));
            assert_eq!($new(4 as $t, 8 as $t), (2 as $t * a));
            assert_eq!($new(1 as $t, 1 as $t), (a / a));
            assert_eq!($new(1 as $t, 2 as $t), (a / 2 as $t));
            assert_eq!($new(2 as $t, 1 as $t), (4 as $t / a));
        }

        #[test]
        fn test_assign_ops() {
            let a = $new(1 as $t, 2 as $t);
            let mut b = a;
            b += a;
            assert_eq!($new(2 as $t, 4 as $t), b);
            b -= a;
            assert_eq!($new(1 as $t, 2 as $t), b);
            b *= a;
            assert_eq!($new(1 as $t, 4 as $t), b);
            b /= a;
            assert_eq!($new(1 as $t, 2 as $t), b);
            b *= 2 as $t;
            assert_eq!($new(2 as $t, 4 as $t), b);
            b /= 2 as $t;
            assert_eq!($new(1 as $t, 2 as $t), b);
        }

        #[test]
        fn test_min_max() {
            let a = $new(0 as $t, 2 as $t);
            let b = $new(1 as $t, 1 as $t);
            assert_eq!($new(0 as $t, 1 as $t), a.min(b));
            assert_eq!($new(0 as $t, 1 as $t), b.min(a));
            assert_eq!($new(1 as $t, 2 as $t), a.max(b));
            assert_eq!($new(1 as $t, 2 as $t), b.max(a));
        }

        #[test]
        fn test_hmin_hmax() {
            let a = $new(1 as $t, 2 as $t);
            assert_eq!(1 as $t, a.min_element());
            assert_eq!(2 as $t, a.max_element());
        }

        #[test]
        fn test_eq() {
            let a = $new(1 as $t, 1 as $t);
            let b = $new(1 as $t, 2 as $t);
            assert!(a.cmpeq(a).all());
            assert!(b.cmpeq(b).all());
            assert!(a.cmpne(b).any());
            assert!(b.cmpne(a).any());
            assert!(b.cmpeq(a).any());
        }

        #[test]
        fn test_cmp() {
            assert!(!$mask::default().any());
            assert!(!$mask::default().all());
            assert_eq!($mask::default().bitmask(), 0x0);
            let a = $new(1 as $t, 1 as $t);
            let b = $new(2 as $t, 2 as $t);
            let c = $new(1 as $t, 1 as $t);
            let d = $new(2 as $t, 1 as $t);
            assert_eq!(a.cmplt(a).bitmask(), 0x0);
            assert_eq!(a.cmplt(b).bitmask(), 0x3);
            assert_eq!(a.cmplt(d).bitmask(), 0x1);
            assert_eq!(c.cmple(a).bitmask(), 0x3);
            assert!(a.cmplt(b).all());
            assert!(a.cmplt(d).any());
            assert!(a.cmple(b).all());
            assert!(a.cmple(a).all());
            assert!(b.cmpgt(a).all());
            assert!(b.cmpge(a).all());
            assert!(b.cmpge(b).all());
            assert!(!(a.cmpge(d).all()));
            assert!(c.cmple(c).all());
            assert!(c.cmpge(c).all());
            assert!(a == a);
            assert!(a < b);
            assert!(b > a);
        }

        #[test]
        fn test_extend_truncate() {
            let a = $new(1 as $t, 2 as $t);
            let b = a.extend(3 as $t);
            assert_eq!($vec3::new(1 as $t, 2 as $t, 3 as $t), b);
        }

        #[test]
        fn test_vec2mask() {
            // make sure the unused 'z' value doesn't break $vec2 behaviour
            let a = $vec3::zero();
            let mut b = a.truncate();
            b.x = 1 as $t;
            b.y = 1 as $t;
            assert!(!b.cmpeq($vec2::zero()).any());
            assert!(b.cmpeq($vec2::splat(1 as $t)).all());
        }

        #[test]
        fn test_mask_as_ref() {
            assert_eq!($mask::new(false, false).as_ref(), &[0, 0]);
            assert_eq!($mask::new(true, false).as_ref(), &[!0, 0]);
            assert_eq!($mask::new(false, true).as_ref(), &[0, !0]);
            assert_eq!($mask::new(true, true).as_ref(), &[!0, !0]);
        }

        #[test]
        fn test_mask_from() {
            assert_eq!(Into::<[u32; 2]>::into($mask::new(false, false)), [0, 0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(true, false)), [!0, 0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(false, true)), [0, !0]);
            assert_eq!(Into::<[u32; 2]>::into($mask::new(true, true)), [!0, !0]);
        }

        #[test]
        fn test_mask_bitmask() {
            assert_eq!($mask::new(false, false).bitmask(), 0b00);
            assert_eq!($mask::new(true, false).bitmask(), 0b01);
            assert_eq!($mask::new(false, true).bitmask(), 0b10);
            assert_eq!($mask::new(true, true).bitmask(), 0b11);
        }

        #[test]
        fn test_mask_any() {
            assert_eq!($mask::new(false, false).any(), false);
            assert_eq!($mask::new(true, false).any(), true);
            assert_eq!($mask::new(false, true).any(), true);
            assert_eq!($mask::new(true, true).any(), true);
        }

        #[test]
        fn test_mask_all() {
            assert_eq!($mask::new(false, false).all(), false);
            assert_eq!($mask::new(true, false).all(), false);
            assert_eq!($mask::new(false, true).all(), false);
            assert_eq!($mask::new(true, true).all(), true);
        }

        #[test]
        fn test_mask_select() {
            let a = $vec2::new(1 as $t, 2 as $t);
            let b = $vec2::new(3 as $t, 4 as $t);
            assert_eq!(
                $vec2::select($mask::new(true, true), a, b),
                $vec2::new(1 as $t, 2 as $t),
            );
            assert_eq!(
                $vec2::select($mask::new(true, false), a, b),
                $vec2::new(1 as $t, 4 as $t),
            );
            assert_eq!(
                $vec2::select($mask::new(false, true), a, b),
                $vec2::new(3 as $t, 2 as $t),
            );
            assert_eq!(
                $vec2::select($mask::new(false, false), a, b),
                $vec2::new(3 as $t, 4 as $t),
            );
        }

        #[test]
        fn test_mask_and() {
            assert_eq!(
                ($mask::new(false, false) & $mask::new(false, false)).bitmask(),
                0b00,
            );
            assert_eq!(
                ($mask::new(true, true) & $mask::new(true, false)).bitmask(),
                0b01,
            );
            assert_eq!(
                ($mask::new(true, false) & $mask::new(false, true)).bitmask(),
                0b00,
            );
            assert_eq!(
                ($mask::new(true, true) & $mask::new(true, true)).bitmask(),
                0b11,
            );

            let mut mask = $mask::new(true, true);
            mask &= $mask::new(true, false);
            assert_eq!(mask.bitmask(), 0b01);
        }

        #[test]
        fn test_mask_or() {
            assert_eq!(
                ($mask::new(false, false) | $mask::new(false, false)).bitmask(),
                0b00,
            );
            assert_eq!(
                ($mask::new(false, false) | $mask::new(false, true)).bitmask(),
                0b10,
            );
            assert_eq!(
                ($mask::new(true, false) | $mask::new(false, true)).bitmask(),
                0b11,
            );
            assert_eq!(
                ($mask::new(true, true) | $mask::new(true, true)).bitmask(),
                0b11,
            );

            let mut mask = $mask::new(true, true);
            mask |= $mask::new(true, false);
            assert_eq!(mask.bitmask(), 0b11);
        }

        #[test]
        fn test_mask_not() {
            assert_eq!((!$mask::new(false, false)).bitmask(), 0b11);
            assert_eq!((!$mask::new(true, false)).bitmask(), 0b10);
            assert_eq!((!$mask::new(false, true)).bitmask(), 0b01);
            assert_eq!((!$mask::new(true, true)).bitmask(), 0b00);
        }

        #[test]
        fn test_mask_fmt() {
            let a = $mask::new(true, false);

            assert_eq!(
                format!("{:?}", a),
                format!("{}(0xffffffff, 0x0)", stringify!($mask))
            );
            assert_eq!(format!("{}", a), "[true, false]");
        }

        #[test]
        fn test_mask_eq() {
            let a = $mask::new(true, false);
            let b = $mask::new(true, false);
            let c = $mask::new(false, true);

            assert_eq!(a, b);
            assert_eq!(b, a);
            assert_ne!(a, c);
            assert_ne!(b, c);

            assert!(a > c);
            assert!(c < a);
        }

        #[test]
        fn test_mask_hash() {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hash;
            use std::hash::Hasher;

            let a = $mask::new(true, false);
            let b = $mask::new(true, false);
            let c = $mask::new(false, true);

            let mut hasher = DefaultHasher::new();
            a.hash(&mut hasher);
            let a_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            b.hash(&mut hasher);
            let b_hashed = hasher.finish();

            let mut hasher = DefaultHasher::new();
            c.hash(&mut hasher);
            let c_hashed = hasher.finish();

            assert_eq!(a, b);
            assert_eq!(a_hashed, b_hashed);
            assert_ne!(a, c);
            assert_ne!(a_hashed, c_hashed);
        }

        #[test]
        fn test_to_from_slice() {
            let v = $vec2::new(1 as $t, 2 as $t);
            let mut a = [0 as $t, 0 as $t];
            v.write_to_slice_unaligned(&mut a);
            assert_eq!(v, $vec2::from_slice_unaligned(&a));
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_sum() {
            let one = $vec2::one();
            assert_eq!(vec![one, one].iter().sum::<$vec2>(), one + one);
        }

        #[cfg(feature = "std")]
        #[test]
        fn test_product() {
            let two = $vec2::new(2 as $t, 2 as $t);
            assert_eq!(vec![two, two].iter().product::<$vec2>(), two * two);
        }
    };
}

macro_rules! impl_vec2_signed_tests {
    ($t:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident) => {
        impl_vec2_tests!($t, $new, $vec2, $vec3, $mask);

        #[test]
        fn test_dot_signed() {
            let x = $new(1 as $t, 0 as $t);
            let y = $new(0 as $t, 1 as $t);
            assert_eq!(1 as $t, x.dot(x));
            assert_eq!(0 as $t, x.dot(y));
            assert_eq!(-1 as $t, x.dot(-x));
        }

        #[test]
        fn test_neg() {
            let a = $new(1 as $t, 2 as $t);
            assert_eq!($new(-1 as $t, -2 as $t), (-a));
        }
    };
}

macro_rules! impl_vec2_float_tests {
    ($t:ident, $new:ident, $vec2:ident, $vec3:ident, $mask:ident, $mat2:ident) => {
        impl_vec2_signed_tests!($t, $new, $vec2, $vec3, $mask);

        use core::$t::INFINITY;
        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        #[test]
        fn test_length() {
            let x = $new(1.0, 0.0);
            let y = $new(0.0, 1.0);
            assert_eq!(4.0, (2.0 * x).length_squared());
            assert_eq!(9.0, (-3.0 * y).length_squared());
            assert_eq!(2.0, (-2.0 * x).length());
            assert_eq!(3.0, (3.0 * y).length());
            assert_eq!(2.0, x.distance_squared(y));
            assert_eq!(13.0, (2.0 * x).distance_squared(-3.0 * y));
            assert_eq!((2.0 as $t).sqrt(), x.distance(y));
            assert_eq!(5.0, (3.0 * x).distance(-4.0 * y));
            assert_eq!(13.0, (-5.0 * x).distance(12.0 * y));
            assert_eq!(x, (2.0 * x).normalize());
            assert_eq!(1.0 * 3.0 + 2.0 * 4.0, $new(1.0, 2.0).dot($new(3.0, 4.0)));
            assert_eq!(2.0 * 2.0 + 3.0 * 3.0, $new(2.0, 3.0).length_squared());
            assert_eq!(
                (2.0 as $t * 2.0 + 3.0 * 3.0).sqrt(),
                $new(2.0, 3.0).length()
            );
            assert_eq!(
                1.0 / (2.0 as $t * 2.0 + 3.0 * 3.0).sqrt(),
                $new(2.0, 3.0).length_recip()
            );
            assert!($new(2.0, 3.0).normalize().is_normalized());
            assert_eq!(
                $new(2.0, 3.0) / (2.0 as $t * 2.0 + 3.0 * 3.0).sqrt(),
                $new(2.0, 3.0).normalize()
            );
            assert_eq!($new(0.5, 0.25), $new(2.0, 4.0).recip());
        }

        #[test]
        fn test_perp() {
            let v1 = $vec2::new(1.0, 2.0);
            let v2 = $vec2::new(1.0, 1.0);
            let v1_perp = $vec2::new(-2.0, 1.0);
            let rot90 = $mat2::from_angle($t::to_radians(90.0));

            assert_eq!(v1_perp, v1.perp());
            assert_eq!(v1.perp().dot(v1), 0.0);
            assert_eq!(v2.perp().dot(v2), 0.0);
            assert_eq!(v1.perp().dot(v2), v1.perp_dot(v2));

            assert_approx_eq!(v1.perp(), rot90 * v1);
        }

        #[test]
        fn test_sign() {
            assert_eq!($vec2::zero().signum(), $vec2::one());
            assert_eq!(-$vec2::zero().signum(), -$vec2::one());
            assert_eq!($vec2::one().signum(), $vec2::one());
            assert_eq!((-$vec2::one()).signum(), -$vec2::one());
            assert_eq!($vec2::splat(INFINITY).signum(), $vec2::one());
            assert_eq!($vec2::splat(NEG_INFINITY).signum(), -$vec2::one());
            assert!($vec2::splat(NAN).signum().is_nan_mask().all());
        }

        #[test]
        fn test_abs() {
            assert_eq!($vec2::zero().abs(), $vec2::zero());
            assert_eq!($vec2::one().abs(), $vec2::one());
            assert_eq!((-$vec2::one()).abs(), $vec2::one());
        }

        #[test]
        fn test_round() {
            assert_eq!($vec2::new(1.35, 0.0).round().x, 1.0);
            assert_eq!($vec2::new(0.0, 1.5).round().y, 2.0);
            assert_eq!($vec2::new(0.0, -15.5).round().y, -16.0);
            assert_eq!($vec2::new(0.0, 0.0).round().y, 0.0);
            assert_eq!($vec2::new(0.0, 21.1).round().y, 21.0);
            assert_eq!($vec2::new(0.0, 11.123).round().y, 11.0);
            assert_eq!($vec2::new(0.0, 11.499).round().y, 11.0);
            assert_eq!(
                $vec2::new(NEG_INFINITY, INFINITY).round(),
                $vec2::new(NEG_INFINITY, INFINITY)
            );
            assert!($vec2::new(NAN, 0.0).round().x.is_nan());
        }

        #[test]
        fn test_floor() {
            assert_eq!($vec2::new(1.35, -1.5).floor(), $vec2::new(1.0, -2.0));
            assert_eq!(
                $vec2::new(INFINITY, NEG_INFINITY).floor(),
                $vec2::new(INFINITY, NEG_INFINITY)
            );
            assert!($vec2::new(NAN, 0.0).floor().x.is_nan());
            assert_eq!(
                $vec2::new(-2000000.123, 10000000.123).floor(),
                $vec2::new(-2000001.0, 10000000.0)
            );
        }

        #[test]
        fn test_ceil() {
            assert_eq!($vec2::new(1.35, -1.5).ceil(), $vec2::new(2.0, -1.0));
            assert_eq!(
                $vec2::new(INFINITY, NEG_INFINITY).ceil(),
                $vec2::new(INFINITY, NEG_INFINITY)
            );
            assert!($vec2::new(NAN, 0.0).ceil().x.is_nan());
            assert_eq!(
                $vec2::new(-2000000.123, 1000000.123).ceil(),
                $vec2::new(-2000000.0, 1000001.0)
            );
        }

        #[test]
        fn test_lerp() {
            let v0 = $vec2::new(-1.0, -1.0);
            let v1 = $vec2::new(1.0, 1.0);
            assert_approx_eq!(v0, v0.lerp(v1, 0.0));
            assert_approx_eq!(v1, v0.lerp(v1, 1.0));
            assert_approx_eq!($vec2::zero(), v0.lerp(v1, 0.5));
        }

        #[test]
        fn test_is_finite() {
            assert!($vec2::new(0.0, 0.0).is_finite());
            assert!($vec2::new(-1e-10, 1e10).is_finite());
            assert!(!$vec2::new(INFINITY, 0.0).is_finite());
            assert!(!$vec2::new(0.0, NAN).is_finite());
            assert!(!$vec2::new(0.0, NEG_INFINITY).is_finite());
            assert!(!$vec2::new(INFINITY, NEG_INFINITY).is_finite());
        }

        #[test]
        fn test_powf() {
            assert_eq!($vec2::new(2.0, 4.0).powf(2.0), $vec2::new(4.0, 16.0));
        }

        #[test]
        fn test_exp() {
            assert_eq!(
                $vec2::new(1.0, 2.0).exp(),
                $vec2::new((1.0 as $t).exp(), (2.0 as $t).exp())
            );
        }

        #[test]
        fn test_angle_between() {
            let angle = $vec2::new(1.0, 0.0).angle_between($vec2::new(0.0, 1.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_2, angle, 1e-6);

            let angle = $vec2::new(10.0, 0.0).angle_between($vec2::new(0.0, 5.0));
            assert_approx_eq!(core::$t::consts::FRAC_PI_2, angle, 1e-6);

            let angle = $vec2::new(-1.0, 0.0).angle_between($vec2::new(0.0, 1.0));
            assert_approx_eq!(-core::$t::consts::FRAC_PI_2, angle, 1e-6);
        }
    };
}

mod vec2 {
    use glam::{vec2, Mat2, Vec2, Vec2Mask, Vec3};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(8, mem::size_of::<Vec2>());
        assert_eq!(4, mem::align_of::<Vec2>());
        assert_eq!(8, mem::size_of::<Vec2Mask>());
        assert_eq!(4, mem::align_of::<Vec2Mask>());
    }

    impl_vec2_float_tests!(f32, vec2, Vec2, Vec3, Vec2Mask, Mat2);
}

mod dvec2 {
    use glam::{dvec2, DMat2, DVec2, DVec3, UVec2Mask};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(16, mem::size_of::<DVec2>());
        assert_eq!(8, mem::align_of::<DVec2>());
        assert_eq!(8, mem::size_of::<UVec2Mask>());
        assert_eq!(4, mem::align_of::<UVec2Mask>());
    }

    impl_vec2_float_tests!(f64, dvec2, DVec2, DVec3, UVec2Mask, DMat2);
}

mod ivec2 {
    use glam::{ivec2, IVec2, IVec3, UVec2Mask};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(8, mem::size_of::<IVec2>());
        assert_eq!(4, mem::align_of::<IVec2>());
        assert_eq!(8, mem::size_of::<UVec2Mask>());
        assert_eq!(4, mem::align_of::<UVec2Mask>());
    }

    impl_vec2_signed_tests!(i32, ivec2, IVec2, IVec3, UVec2Mask);
}

mod uvec2 {
    use glam::{uvec2, UVec2, UVec2Mask, UVec3};

    #[test]
    fn test_align() {
        use core::mem;
        assert_eq!(8, mem::size_of::<UVec2>());
        assert_eq!(4, mem::align_of::<UVec2>());
        assert_eq!(8, mem::size_of::<UVec2Mask>());
        assert_eq!(4, mem::align_of::<UVec2Mask>());
    }

    impl_vec2_tests!(u32, uvec2, UVec2, UVec3, UVec2Mask);
}

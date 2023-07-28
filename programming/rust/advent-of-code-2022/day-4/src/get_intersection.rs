#[allow(non_camel_case_types)]
enum _RangeIntersection {
    // unit unit
    ab_cd,
    abcd,
    cd_ab,

    // unit n
    ab_c_d,
    abc_d,
    c_ab_d,
    c_abd,
    c_d_ab,

    // n unit
    cd_a_b,
    acd_b,
    a_cd_b,
    a_bcd,
    a_b_cd,

    // n n
    a_b_c_d,
    a_bc_d,
    a_c_b_d,
    ac_bd,
    c_a_bd,
    ac_b_d,
    a_c_bd,
    ac_d_b,
    c_a_b_d,
    a_c_d_b,
    c_a_d_b,
    c_ad_b,
    c_d_a_b,
}
//use RangeIntersection::*;

pub fn get_intersection(pair: ((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
    match pair {
        // unit ranges
        ((a, b), (c, d)) if a == b || c == d => match ((a, b), (c, d)) {
            // both units, means they must be equal to overlap
            ((a, b), (c, d)) if a == b && c == d => match ((a, b), (c, d)) {
                (x, y) if x == y => Some(x),
                _ => None,
            },

            // (a,b) unit
            ((a, b), (c, d)) if a == b && c != d => match ((a, b), (c, d)) {
                ((a, b), (c, d)) if a >= c && a <= d => Some((a, b)),
                _ => None,
            },

            // (c,d) unit
            ((a, b), (c, d)) if a != b && c == d => match ((a, b), (c, d)) {
                ((a, b), (c, d)) if c >= a && c <= b => Some((c, d)),
                _ => None,
            },

            _ => {
                println!("Unreachable code reached in get_intersection");
                None
            }
        },

        // equal non-unit range
        ((a, b), (c, d)) if (a, b) == (c, d) => Some((a, b)),

        // |a,b| < |c,d|
        ((a, b), (c, d)) if a < c && b == c && c < d => Some((b, c)),
        ((a, b), (c, d)) if a < c && c < b && b < d => Some((c, b)),
        ((a, b), (c, d)) if a == c && c < b && b < d => Some((a, b)),
        ((a, b), (c, d)) if c < a && a < b && b < d => Some((a, b)),
        ((a, b), (c, d)) if c < a && a < b && b == d => Some((a, b)),
        ((a, b), (c, d)) if c < a && a < d && d < b => Some((a, d)),
        ((a, b), (c, d)) if c < a && a == d && d < b => Some((a, d)),

        // |a,b| > |c,d|
        ((c, d), (a, b)) if a < c && b == c && c < d => Some((b, c)),
        ((c, d), (a, b)) if a < c && c < b && b < d => Some((c, b)),
        ((c, d), (a, b)) if a == c && c < b && b < d => Some((a, b)),
        ((c, d), (a, b)) if c < a && a < b && b < d => Some((a, b)),
        ((c, d), (a, b)) if c < a && a < b && b == d => Some((a, b)),
        ((c, d), (a, b)) if c < a && a < d && d < b => Some((a, d)),
        ((c, d), (a, b)) if c < a && a == d && d < b => Some((a, d)),

        // no overlap
        _ => None,
    }
}

fn _get_intersection_old(pair: ((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
    // this is simply awful, but I can't think of a better way to do it
    // this way makes it WAY too easy to miss cases, as I found out the
    // hard way.
    match pair {
        ((a, b), (c, d)) if a < c && c < b && b < d => Some((c, b)),
        ((a, b), (c, d)) if a == c && c < b && b < d => Some((a, b)),
        ((a, b), (c, d)) if a == c && c < b && b == d => Some((a, b)),
        ((a, b), (c, d)) if c < a && c < b && b == d => Some((a, b)),
        ((a, b), (c, d)) if c < a && a < d && d < b => Some((a, d)),
        ((c, d), (a, b)) if a < c && c < b && b < d => Some((c, b)),
        ((c, d), (a, b)) if a == c && c < b && b < d => Some((a, b)),
        ((c, d), (a, b)) if a == c && c < b && b == d => Some((a, b)),
        ((c, d), (a, b)) if c < a && c < b && b == d => Some((a, b)),
        ((c, d), (a, b)) if c < a && a < d && d < b => Some((a, d)),
        ((a, b), (c, d)) if a < c && b > d => Some((c, d)),
        ((c, d), (a, b)) if a < c && b > d => Some((c, d)),
        ((a, b), (c, d)) if a == b && a >= c && a <= d => Some((a, b)),
        ((c, d), (a, b)) if a == b && a >= c && a <= d => Some((a, b)),
        _ => None,
    }
}

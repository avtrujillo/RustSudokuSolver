#[macro_export]
macro_rules! array_from_take {
    ($source_iter:ident, $take_count:ident) => {
        {let mut take_iter = $source_iter.take($take_count).peekable();
        let mut output_arr = [*(take_iter.peek().expect()); $take_count];
        for (ind) in (0..($take_count - 1)) {
            output_arr[ind] = take_iter.next();
        }
        output_arr}
    }
}

#[macro_use]
pub(crate) use array_from_take;

#[macro_export]
macro_rules!twenty_seven {
    ($to_repeat:expr) => {
        [
            $to_repeat, $to_repeat, $to_repeat, $to_repeat, $to_repeat,
            $to_repeat, $to_repeat, $to_repeat, $to_repeat, $to_repeat,
            $to_repeat, $to_repeat, $to_repeat, $to_repeat, $to_repeat,
            $to_repeat, $to_repeat, $to_repeat, $to_repeat, $to_repeat,
            $to_repeat, $to_repeat, $to_repeat, $to_repeat, $to_repeat,
            $to_repeat, $to_repeat
        ]
    }
}

#[macro_use]
pub(crate) use twenty_seven;

/*
#[macro_export]
macro_rules! array_from_block_over_range {
    ($source_range:ident, $some_block:block) => {
        {let output_iter = $source_range.map($some_block);
        let output_length = $source_range.len();
        array_from_take!(output_iter, output_length)}
    }
}

#[macro_use]
pub(crate) use crate::sd_macros::array_from_block_over_range;

#[macro_export]
macro_rules! n_element_filter {
    ($source_arr:ident, $output_size:ident, $filter_block:block) => {
        let source_length = $source_arr.len();
        let mut filter_iter = $source_arr.into_iter().filter($filter_block).peekable();
        let mut output_arr = [filter_iter.peek(); $output_size]
        let mut output_ind = 0;
        filter_iter.for_each ( |elem|
            match elem {
                Some(some_elem) => {
                    output_arr[output_ind] = some_elem;
                    output_ind += 1
                }
                None => {
                    //noop
                }
            }
        )
        if output_ind != $output_size {
            panic!("n_element_filter produced more than n elements")
        }
        output_arr
    }
}

#[macro_use]
pub(crate) use crate::sd_macros::n_element_filter;
*/
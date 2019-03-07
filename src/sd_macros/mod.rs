#[macro_export]
macro_rules! array_from_take {
    ($source_iter:ident<$coll_type:ty>, $take_count:ident) => {
        let mut take_iter = $source_iter.take($take_count);
        let mut output_arr = [$coll_type::new(); $take_count];
        for (ind, elem) in (0..($take_count - 1)).enumerate() {
            output_arr[ind] = elem;
        }
        output_arr
    }
}

#[macro_export]
macro_rules! array_from_block_over_range {
    ($source_range:item, $some_block:block) => {
        source_iter = for n in $source_range $some_block;
        array_from_take!(source_iter, $source_range.len())
    }
}

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
/// Some specifications for the vector module.
spec std::vector {
    

    
    /// Slice spec.
    spec slice {
        // aborts only if:
        aborts_if start > end;
        aborts_if end > len(v);
        // A ghost variable
        global old_start : u64;
        // Lentgh of the slice
        ensures len(result) == end - start;
        // The slice is a subsequence of the original vector
        ensures forall i: u64 where 0 <= i && i < len(result) : result[i] == v[start + i];
    }
}

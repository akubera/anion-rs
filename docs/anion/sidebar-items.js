initSidebarItems({"enum":[["AnionValue","Enum of all possible types of elements in an ion document."],["Error","\"Master\" Error type enumerating all possible failures when dealing with anion types."],["NonNullAnionValue","Variant of AnionValue enum that does not permit null values. This includes the 'pure NULL' null value, though this may be removed due to naming sillyness. This is much closer in type to true JSON values."]],"fn":[["from","Return equivalent AnionValue from some object. Convenience for `AnionValue::from(..)`."],["parse","Parse a string and try to create an AnionValue. Convenience method for `str::parse::<AnionValue>(..)`."]],"macro":[["ion_list","Macro to easily build an <a class=\"type\" href=\"type.AnionList.html\">AnionList</a>; similiar to `vec![]`"]],"mod":[["parser",""]],"type":[["AnionBlob","Raw binary data"],["AnionBool","Simple boolean value - mapped directly to rust's *bool*"],["AnionDecimal","Exact decimal"],["AnionFloat","64-bit floating point number"],["AnionInt","Unbounded integer"],["AnionList","List of AnionValue items"],["AnionString","Unicode characters"],["AnionStruct","Map of symbols to arbitrary AnionValues"],["AnionSymbol","Unicode characters - may be used for keys and symbols (not values)"],["AnionTimestamp","point in time"],["Result","Result wrapping all possible errors in this crate"]]});
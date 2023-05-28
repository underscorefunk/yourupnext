- [] CmdResult should not accept a generic. It should be stuck to Result<State, SomeErr>. Other results that dob't have
  State as their Ok value should be a generic result type.
- [] Create a shared "App Error" type and apply it to queries and Commands.


TERSE API
--------------
'd'   // data (looks like line from above)
'q'   // query (looks like line from below)
'u'   // update (looks like a container)
'uu'  // update many 

DATA SOURCE
--------------
data() <- a closure that enables apply with
query() <- a closure tha enables apply with

STATE CHANGE
--------------
apply() <- directly applies an applicator to the stte
apply_with()
  on a vec, it will loop


data() -> vec<T>
query() -> vec<T>


  for a query that is a vector
  query that is a vector
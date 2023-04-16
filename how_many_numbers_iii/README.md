We want to generate all the numbers of three digits where:

- the sum of their digits is equal to `10`
- their digits are in increasing order (the numbers may have two or more equal contiguous digits)

The numbers that fulfill these constraints are: `[118, 127, 136, 145, 226, 235, 244, 334]`. There're `8` numbers in total with `118` being the lowest and `334` being the greatest.

___

## Task

Implement a function which receives two arguments:

1. the sum of digits (`sum`)
2. the number of digits (`count`)

This function should return three values:

1. the total number of values which have `count` digits that add up to `sum` and are in increasing order
2. the lowest such value
3. the greatest such value

**Note**: if there're no values which satisfy these constaints, you should return an empty value (refer to the examples to see what exactly is expected).

## Examples

```python
find_all(10, 3)  =>  [8, 118, 334]
find_all(27, 3)  =>  [1, 999, 999]
find_all(84, 4)  =>  []
```
```ruby
find_all(10, 3)  =>  [8, 118, 334]
find_all(27, 3)  =>  [1, 999, 999]
find_all(84, 4)  =>  []
```
```crystal
find_all(10, 3)  =>  [8, 118, 334]
find_all(27, 3)  =>  [1, 999, 999]
find_all(84, 4)  =>  []
```
```javascript
findAll(10, 3)  =>  [8, "118", "334"]
findAll(27, 3)  =>  [1, "999", "999"]
findAll(84, 4)  =>  []
```
```haskell
findAll 10 3  =>  ( 8, Just 118, Just 334 )
findAll 27 3  =>  ( 1, Just 999, Just 999 )
findAll 84 4  =>  ( 0, Nothing, Nothing )
```
```java
// The output type is List<Long>
HowManyNumbers.findAll(10, 3)  =>  [8, 118, 334]
HowManyNumbers.findAll(27, 3)  =>  [1, 999, 999]
HowManyNumbers.findAll(84, 4)  =>  []
```
```csharp
// The output type is List<long>
HowManyNumbers.FindAll(10, 3)  =>  [8, 118, 334]
HowManyNumbers.FindAll(27, 3)  =>  [1, 999, 999]
HowManyNumbers.FindAll(84, 4)  =>  []
```
```cpp
// The output type is optional<tuple<uint32_t, uint64_t, uint64_t>>
find_all(10, 3)  =>  (8, 118, 334)
find_all(27, 3)  =>  (1, 999, 999)
find_all(84, 4)  =>  nullopt
```

___

Features of the random tests:

* Number of tests: `112`
* Sum of digits value between `20` and `65`
* Amount of digits between `2` and `17`
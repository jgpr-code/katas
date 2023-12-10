let primeFactors x =
    []

let isPrime x =
    false

let isPrime_1_False () =
    assert (not (isPrime 1))

let isPrime_3_True () =
    assert (isPrime 3)


// tests
printfn "Running tests"
isPrime_1_False ()
isPrime_3_True ()
printfn "Done"

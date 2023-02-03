open System

let lines =
    (fun _ -> Console.ReadLine())
    |> Seq.initInfinite
    |> Seq.takeWhile ((<>) null)
    |> Seq.toArray

let buildMemory (line: string) =
    line.Split(",") |> Array.map (fun s -> int (s))

let printMem (memory: int[]) =
    for i in memory do
        printf $"{i}, "

    printfn ""

let rec execMemory (cursor: int) (memory: int[]) =
    match memory[cursor] with
    | 1 ->
        memory[memory[cursor + 3]] <- memory[memory[cursor + 1]] + memory[memory[cursor + 2]]
        execMemory (cursor + 4) memory
    | 2 ->
        memory[memory[cursor + 3]] <- memory[memory[cursor + 1]] * memory[memory[cursor + 2]]
        execMemory (cursor + 4) memory
    | 99 ->
        printfn "Success"
        memory
    | _ ->
        printfn "Error happened"
        memory

let results = lines |> Array.map (fun line -> line |> buildMemory |> execMemory 0)

for result in results do
    for i in result do
        printf $"{i}, "

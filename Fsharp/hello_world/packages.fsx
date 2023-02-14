#r "nuget: FSharp.Data, Version=4.1.1"

open FSharp.Data

let html = Http.RequestString("https://docs.microsoft.com/dotnet/fsharp")

printfn $"{html}"

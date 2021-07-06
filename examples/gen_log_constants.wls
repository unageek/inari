#!/usr/bin/env wolframscript

Import[FileNameJoin @ {DirectoryName[$InputFileName], "..", "tools", "interval.wl"}];

print[value_] :=
    Print[intervalExpression[value] <> ","];

n = 22;

(* bl *)
print[2 Sqrt[2] - 2];

Print["----"];

(* 1/√2 bl *)
print[4 - 2 Sqrt[2]];

Print["----"];

(* initial value of y *)
print[(-1)^(n + 1) / (n Log[2]) (2 Sqrt[2] - 2)^-n];
print[(-1)^(n + 1) / (n Log[2]) (4 - 2 Sqrt[2])^-n];

Print["----"];

(* Taylor coefficients *)
print[0];
For[i = 1, i < n, i++,
  print[(-1)^(i + 1) / (i Log[2])];
 ]

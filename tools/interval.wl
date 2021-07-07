interval[x_] :=
    Module[ {max, ulp, inf, sup},
        max = SetPrecision[$MaxMachineNumber, Infinity];
        ulp = 2^(Max[Floor[Log2[Abs[x]]] - 52, -1074]);
        inf = If[ x < -max,
                  -Infinity,
                  N @ Floor[x, ulp]
              ];
        sup = If[ x > max,
                  Infinity,
                  N @ Ceiling[x, ulp]
              ];
        {inf, sup}
    ];

f64Expression[x_] :=
    Switch[x,
        Infinity, "f64::INFINITY",
        -Infinity, "f64::NEG_INFINITY",
        _, ExportString[x, "JSON"]
    ];

intervalExpression[x_] :=
    Module[ {inf, sup},
        {inf, sup} = f64Expression /@ interval[x];
        "const_interval!(" <> inf <> "," <> sup <> ")"
    ];

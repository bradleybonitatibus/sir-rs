# sir-rs

![CI](https://github.com/bradleybonitatibus/sir-rs/workflows/CI/badge.svg)

SIR infectious disease modelling CLI

## Example Usage
The CLI takes 5 input parameters to the application to run the simulation.
- Recovery Rate (`gamma`)
- Infection Rate (`beta`)
- Time Steps (`t`)
- Initial Susceptible (S<sub>0</sub>)
- Initial Infectious (I<sub>0</sub>)
- Initial Recovered (R<sub>0</sub>)


```bash
cargo run -- {gamma} {beta} {t} {S} {I} {R}
```

Note: Please replace the `{}` values with your corresponding values.

## Output
The results will be output to a CSV locally and will have the following columns (in order):

- Time Step / Day
- S
- I
- R

```csv
0,10000,1000,0
1,9000,1950,50
2,7245,3607.5,147.5
3,4631.36625,6040.75875,327.875
4,1833.6696300857811,8536.417432414219,629.9129375
5,268.3726905305018,9674.893500348788,1056.733809120711
6,8.724970602034944,9450.796545259815,1540.4784841381504
7,0.47917839971441367,8986.502510199145,2013.018311401141
8,0.048564610527734864,8537.607998478374,2462.3434369110983
9,0.007102049799277241,8110.769061115184,2889.223836835017
10,0.0013417412210295259,7705.236368368003,3294.7622898907766
```

## Model

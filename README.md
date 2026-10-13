# ABM Frameworks Comparison

Benchmark simulation implementations for comparing agent-based modeling (ABM)
frameworks across five programming languages. The study evaluates framework
capabilities, execution time, and operating memory usage to help researchers and
practitioners select a framework for their modeling needs.

## Publication

This repository accompanies **Framework Showdown: A Practical Benchmarking Study
of Agent-Based Modeling Platforms**, by Antoni Zięciak, Ewelina Badeja, Joanna
Wójcicka, Michał Szczygieł, Przemysław Orlikowski, and Jarosław Wąs.

Published July 2, 2026, in *Intelligent Agents in Science and Engineering*
(IASE 2025), *Communications in Computer and Information Science*, volume 2976,
pages 31–47, Springer, Cham.

[Read the chapter on Springer](https://link.springer.com/chapter/10.1007/978-3-032-28263-7_3)
· [DOI: 10.1007/978-3-032-28263-7_3](https://doi.org/10.1007/978-3-032-28263-7_3)

### Abstract

In recent years, a wide range of agent-based modeling (ABM) frameworks has been
developed. Many of these advanced tools support the creation of complex
simulations and offer distinct features for various use cases. This practical
comparative study evaluates the capabilities, execution time performance, and
operating memory usage of several ABM frameworks through a series of benchmark
simulations. The analysis centers on five agent-based modeling (ABM) tools, each
correlated with a distinct and widely-used programming language: krABMaga,
selected for its computational efficiency; Mesa, designed for its simplicity;
MASON, recognized for its robustness; Agents.jl, noted for its ongoing
maintenance; and Agent Harmony, which seeks to combine simplicity with high
performance. The goal of this work is to provide a clear overview of the
available options and assist researchers and practitioners in selecting the
best-suited framework for their specific modeling needs.

## Frameworks

| Framework | Language | Directory name |
| --- | --- | --- |
| krABMaga | Rust | `krABMaga` |
| Mesa | Python | `MESA` |
| MASON | Java | `MASON` |
| Agents.jl | Julia | `julia` |
| Agent Harmony | C++ | `AgH` |

## Benchmark models

The table below links to the available implementations. MASON links for fake
news and fungi point to their respective branches; other links point to
directories on the default branch. Framework coverage varies by model.

| Model | Available implementations |
| --- | --- |
| [Conway's Game of Life](game_of_life/) | [krABMaga](game_of_life/krABMaga/), [Mesa](game_of_life/MESA/), [MASON](game_of_life/MASON/Life/), [Agents.jl](game_of_life/julia/), [Agent Harmony](game_of_life/AgH/) |
| [Fake news](fake_news/) | [krABMaga](fake_news/krABMaga/), [Mesa](fake_news/MESA/), [MASON (grid)](https://github.com/Tosiekdev/ABM-frameworks-comparison/tree/fake-news%2FMASON/fake_news/MASON/fake_news_grid), [Agent Harmony](fake_news/AgH/) |
| [Fake news — continuous variant](fake_news_continuous/) | [krABMaga](fake_news_continuous/krABMaga/), [Mesa](fake_news_continuous/MESA/), [MASON (continuous network)](https://github.com/Tosiekdev/ABM-frameworks-comparison/tree/fake-news%2FMASON/fake_news/MASON/fake_news_cont_net), [Agent Harmony](fake_news_continuous/AgH/) |
| [Fungi](fungi/) | [krABMaga](fungi/krABMaga/), [Mesa](fungi/MESA/), [MASON](https://github.com/Tosiekdev/ABM-frameworks-comparison/tree/fungi%2FMASON/fungi/MASON), [Agent Harmony](fungi/AgH/) |

### Implementation notes

Model-specific instructions are provided in the implementation READMEs where
available. Game of Life includes Agent Harmony variants
[with visualization](game_of_life/AgH/with_visualization/) and
[without visualization](game_of_life/AgH/without_visualization/), and Agents.jl
variants for the [console](game_of_life/julia/console/) and
[GUI](game_of_life/julia/gui/).

[Agent Harmony](https://github.com/Tosiekdev/Agent-Harmony) is also referenced as
a Git submodule at `Agent-Harmony/`.

## Citation

If you use these benchmark implementations in your research, please cite the
published paper:

Zięciak, A., Badeja, E., Wójcicka, J., Szczygieł, M., Orlikowski, P., & Wąs, J.
(2026). Framework Showdown: A Practical Benchmarking Study of Agent-Based
Modeling Platforms. In K. Mason, U. Haider, & M. E. Cruz Victorio (Eds.),
*Intelligent Agents in Science and Engineering* (IASE 2025), *Communications in
Computer and Information Science*, vol. 2976, pp. 31–47. Springer, Cham.
[https://doi.org/10.1007/978-3-032-28263-7_3](https://doi.org/10.1007/978-3-032-28263-7_3)

```bibtex
@inproceedings{zieciak2026frameworkshowdown,
  author    = {Zięciak, Antoni and Badeja, Ewelina and Wójcicka, Joanna and
               Szczygieł, Michał and Orlikowski, Przemysław and Wąs, Jarosław},
  title     = {Framework Showdown: A Practical Benchmarking Study of Agent-Based Modeling Platforms},
  editor    = {Mason, Karl and Haider, Usman and Cruz Victorio, Marcos Eduardo},
  booktitle = {Intelligent Agents in Science and Engineering},
  series    = {Communications in Computer and Information Science},
  volume    = {2976},
  pages     = {31--47},
  year      = {2026},
  month     = jul,
  publisher = {Springer},
  address   = {Cham},
  note      = {IASE 2025},
  doi       = {10.1007/978-3-032-28263-7_3},
  url       = {https://doi.org/10.1007/978-3-032-28263-7_3}
}
```

## License

The repository code is licensed under the [MIT License](LICENSE).

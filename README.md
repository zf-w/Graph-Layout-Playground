# Zhifeng's Graph Layout Playground

![Jagmesh1 SVG](https://raw.githubusercontent.com/zf-w/zf-w/main/Assets/Jagmesh1.svg)

I'm playing around with a graph layout algorithm. I'm trying to reproduce the results of the Multi-level Force-directed Graph layout algorithm based on Hu, Y. (2005). Efficient, high-quality force-directed graph drawing. Mathematica journal, 10(1), 37-71.

## Introduction

The Multi-level Force-directed algorithm consists of three main sections: graph coarsening, force-directed layout, and graph refinement.

### Graph Coarsen

I chose the Maximal Independent Vertex Set for the graph coarsening part. The general idea is to select a vertex, remove its neighbors, until all the vertices in a graph are labeled as kept or removed, and then build edges between all the pairs of remaining vertices within a distance of 3 in the original graph.

### Force Directed Layout

This part of the algorithm simulates electric-like repulsive forces, which degrade with the square of the physical distance, and spring-like attracting forces, which rise with the physical distance, to make in-graph close vertices closer in their physical position.

### Graph Refinement

This part of the algorithm tries to put back those removed vertices.

## Commands

```
Usage: graph.exe <COMMAND>

Commands:
  coarsen  Perform graph coarsen
  draw     Draw a graph
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### coarsen

```
Usage: graph.exe coarsen [OPTIONS] <graph_json>

Arguments:
  <graph_json>  The path to the JSON file of your input graph. The JSON file should be like {"indices": [0, 1, 1, 2], "position": [1, 1, 2, 2, 3, 3], "dim": 2}.

Options:
  -o, --output-name <out>  The name of the output graph json [default: output.json]
  -d, --depth <depth>      The maximum level of coarsening iteration [default: 1]
  -h, --help               Print help
```

### draw

```
Usage: graph.exe draw [OPTIONS] <graph_json>

Arguments:
  <graph_json>  The path to the JSON file of your input graph. The JSON file should be like {"indices": [0, 1, 1, 2], "position": [1, 1, 2, 2, 3, 3], "dim": 2}.

Options:
  -o, --output-name <out>  The name of the output image [default: output.svg]
  -m, --mass-center
  -w, --width <width>      The width of the output image [default: 1080]
  -h, --help               Print help
```

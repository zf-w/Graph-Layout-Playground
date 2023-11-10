# Zhifeng's Graph Layout Playground

I'm playing around with a graph layout algorithm. I'm trying to reproduce the results of the Multi-level Force-directed Graph layout algorithm based on Hu, Y. (2005). Efficient, high-quality force-directed graph drawing. Mathematica journal, 10(1), 37-71.

## Introduction

The Multi-level Force-directed algorithm consists of three main sections: graph coarsening, force-directed layout, and graph refinement.

### Graph Coarsen

I chose the Maximal Independent Vertex Set for the graph coarsening part. The general idea is to select a vertex, remove its neighbors, until all the vertices in a graph are labeled as kept or removed, and then build edges between all the pairs of remaining vertices within a distance of 3 in the original graph.

### Force Directed Layout

This part of the algorithm simulates electric-like repulsive forces, which degrade with the square of the physical distance, and spring-like attracting forces, which rise with the physical distance, to make in-graph close vertices closer in their physical position.

### Graph Refinement

This part of the algorithm tries to put back those removed vertices.

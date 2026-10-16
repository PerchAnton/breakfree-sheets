"""Iterative deterministic dependency ordering, including blocked cyclic descendants."""

from dataclasses import dataclass
import heapq


@dataclass
class DependencyGraph:
  dependencies: dict[str, frozenset[str]]

  def order(self) -> tuple[list[str], set[str]]:
    nodes = set(self.dependencies)
    degree = {node: len(refs & nodes) for node, refs in self.dependencies.items()}
    dependents = {node: set() for node in nodes}
    for node, refs in self.dependencies.items():
      for ref in refs & nodes:
        dependents[ref].add(node)
    ready = [node for node in nodes if degree[node] == 0]
    heapq.heapify(ready)
    ordered = []
    while ready:
      node = heapq.heappop(ready)
      ordered.append(node)
      for dependent in sorted(dependents[node]):
        degree[dependent] -= 1
        if degree[dependent] == 0:
          heapq.heappush(ready, dependent)
    return ordered, nodes - set(ordered)

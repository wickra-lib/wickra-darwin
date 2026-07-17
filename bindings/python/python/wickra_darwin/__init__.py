"""Wickra Darwin — evolutionary strategy search over Wickra strategy specs.

Construct a :class:`Darwin` from a spec JSON, drive it with command JSONs
(``set_spec``, ``evolve``, ``best``, ``version``), and read back the response
JSON. The same command protocol crosses every language binding, so this Python
front-end drives the exact same core — and returns the byte-identical search —
as the native CLI.
"""

from ._wickra_darwin import Darwin, __version__

__all__ = ["Darwin", "__version__"]

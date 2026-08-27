# Memory Index

- [Market balance index](market-balance-index.md) — every reader/writer of capital / reserved / requested / dispatches in simulation/src
- [Confirmed break families](break-families.md) — the four conservation breaks confirmed at b3857f5 and the shape each belongs to
- [Sold shadow ledger](sold-shadow-ledger.md) — CompanyEnt::sold conserves quantity but grows forever on the 6 store companies (save leak, not a mint)
- [Truck pool reservation lifetime](truck-pool-reservation-lifetime.md) — 15 free sites vs one acquire; free doesn't re-add to the position cache, so same-tick re-acquire misses

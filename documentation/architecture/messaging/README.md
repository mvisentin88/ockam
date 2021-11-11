# Ockam messaging building blocks


## Intro

TODO: intro about messaging
heterogenous networks,
different delivery properties, 
connecting endpoints

End-to-end all the things!

Building blocks for end-to-end communication:

Routing and accessibility

Establishing "ends"

Pipe: unidirectional end-to-end with set properties

Pipe behaviour end definitions

Pipe examples

Pipe combinations: wrapping and injecting



Channel: bidirectional pipe

Request-response communication
Backtracing with guarantees

## Vocabulary and algebra

Route definitions and operations

Route combination and destructuring

Backtraceable routes

Delivery properties in relation to routes

Local assumptions

Routing and worker types, dynamic, static, session, metadata etc.




## Messaging properties

1. Accessibility
1. Reliable delivery
1. Ordering and deduplication

Requirements and limitations when implementing delivery with pipes and channels

Combining different pipes using requirements and limitations

## Implementing pipes, tips and tricks

Asymmetric (multi-address) workers

Session handshakes


## Tolerating errors

Supervision and restarts

Recoverable sessions and monitoring workers

Persistent storage with idempotent worker operations

Wrapping and injecting pipes over unreliable routes


Reliable discovery and static routes








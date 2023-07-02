# LibrePod

A privacy-friendly, self-hosted podcast aggregator optimized for streaming. Although quite a lofty goal, the idea is to work towards building something similar to a PocketCasts open-source replacement in the long-term.

Design-wise, I plan on having the server perform the majority of the heavy lifting (i.e. feed generation, streaming, caching, etc.) for a blazing-fast client-side experience. Not only that, but a server-client architecture allows seamless feed synchronization between multiple devices. It can even enable playback resuming from different clients.

Currently, a basic MVP is being developed with the most useful and familiar features including but not limited to:

- Add subscriptions
- Remove subscriptions
- List subscriptions
- Generate feed based on subscriptions
  - Have multiple auto-generated feed “views”
    - Latest today
    - Latest week (or two weeks)
    - All time (expensive)
- Stream episodes over HTTP efficiently
  - Play “views” (filter out already played)
  - Shuffle option
- Track listening history
- Downloads

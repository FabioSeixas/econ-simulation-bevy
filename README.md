## Agent system with Bevy

Playing around with Bevy

<img width="1009" height="712" alt="image" src="https://github.com/user-attachments/assets/bf0812c2-9323-4e53-8ad8-000613f6a4e0" />

### Next moves (new structure)

- [x] start roles
  - [x] add "No Role" role. The default task is walking around
  - [x] Agent calls role to get next task and actions
- [x] add "Seller" role
  - [x] The default task is selling (using duration)
  - [x] At setup add a lot of MEAT item for this agent
  - [x] Basic Interaction between agents
    - [x] agent with buy action start interaction (send event to seller agent)
    - [x] agent seller receive the event and add it to some internal management where all interactions stay (so multiple buyers can be handled by a single seller)
    - [x] agent seller send event back to buyer. And so on until trade is done.
- [ ] Refactor
  - [ ] split action_frame system into action specific systems
  - [ ] each action specific system will query for Agents with Marker Components (walk_system for Walking, sell_system for Selling etc)
  - [ ] action_frame system can become a "Planning next move" system. It just add the correct Marker Component to Entity based on Agent current action and that is all. Maybe it can query Entities with <Idle> Marker
  - [ ] action completion can be handled in the action specific system, since Agent and Action objects will be there
  - [ ] trade_system queries for entities with TradeNegotiation Component

```rust
#[derive(Component)]
struct TradeNegotiation {
    partner: Entity,
    item: ItemEnum,
    status: TradeStatus,
}
```
  - [ ] remove chain of events for trading interaction. Do not know what to do with general interactions, we will see. Since interaction is only for trade now, just remove everything.


### Next moves (old structure)

- [x] start action system
- [x] decision making inside action system
  - [x] add basic agent needs (hungry, thirst, sleep)
  - [x] add points in the map where needs can be satisfied
  - [x] agent satisfy his need when necessary
- [ ] basic trading (food and water)
  - [x] basic inventory
  - [x] create items "meat" and "water" for now
  - [x] create item "money" ?
  - [x] basic roles ("none" and "seller" for now)
  - [ ] set agent roles on game setup (agent::new())
  - [x] agent system define actions based on agent role (call role system to ask for next role move)
  - [x] agent "do_action" method to spend some time "doing" the action (some work from role)
  - [ ] update agent "complete_current_action" method to when is a work (must call role back to say "I did your task")
  - [ ] agents interaction
    - [ ] agent queue (on action system)
    - [ ] queue action to interact with someone
    - [ ] basic trading (fixed price for now)
- [ ] changes on how to fill needs
  - [ ] when hungry, go buy "meat"
  - [ ] after buy move away before eat
  - [ ] eat must take some time

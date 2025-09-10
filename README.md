## Agent system with Bevy

Playing around with Bevy

<img width="1009" height="712" alt="image" src="https://github.com/user-attachments/assets/bf0812c2-9323-4e53-8ad8-000613f6a4e0" />

### Next moves (current)

- [ ] start roles
  - [ ] add "No Role" role. The default task is walking around
  - [ ] Agent calls role to get next task and actions
- [ ] add "Seller" role
  - [ ] The default task is selling (using duration)
  - [ ] At setup add a lot of MEAT item for this agent
  - [ ] Interaction between agents
    - [ ] agent with buy action start interaction (send event to seller agent)
    - [ ] agent seller receive the event and add it to some internal management where all interactions stay (so multiple buyers can be handled by a single seller)
    - [ ] agent seller send event back to buyer. And so on until trade is done.

### Next moves (old)

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

## Agent system with Bevy

Playing around with Bevy

<img width="1009" height="712" alt="image" src="https://github.com/user-attachments/assets/bf0812c2-9323-4e53-8ad8-000613f6a4e0" />

This is an experiment making a medieval/fantasy economy simulation.

### Imediate next steps

- [ ] Change (all?) interactions event writing (EventWriter) to event triggering (using Commands.trigger()), 
so event handling happens at the exact moment and we do not depend on systems schedulle. 
Hope we can decrease inconsistencies with this.
- [ ] Confirm we do not have more freeze agents (buy task and waiting interaction for ever, for example)

### On the Horizon

- [ ] Collisions
- [ ] Skills -> Define quality of crafts, power for negotiation etc
- [ ] Roles -> Blacksmith, Fisher, Farmer, Livestock Farmer, Hunter, Cook ...
- [ ] Recipes -> Blacksmith knows how to craft, Cook knows how to prepare food etc
- [ ] Orders -> Someone order an iron sword from the Blacksmith, order a soup from the Cook


Changing to use Behavior Trees:

```
? Selector (Acquire Item)
|
+--> -> Sequence (Attempt to Buy from a Known Seller)
|    |
|    +--> ? HasKnownSellersFor(item)? (Condition)
|    |
|    +--> ? Selector (Try Each Known Seller)
|         |
|         +--- *For each seller in knowledge...*
|         |
|         +--> -> Sequence (Go to Seller and Buy)
|              |
|              +--> ? Selector (Ensure Proximity to Seller)
|              |    |
|              |    +--> ? IsNear(seller)? (Condition)
|              |    |
|              |    +--> WalkTo(seller) (Action)
|              |
|              +--> InitiateBuyInteraction(seller) (Action)
|
+--> -> Sequence (Fallback: Find a Seller)
     |
     +--> FindSeller(item) (Action: e.g., start TalkTask)
     |
     +--> Succeed (Action: Ensures the whole tree doesn't stay in a failed state)
```



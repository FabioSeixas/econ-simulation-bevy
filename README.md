## Agent system with Bevy

Playing around with Bevy

<img width="1009" height="712" alt="image" src="https://github.com/user-attachments/assets/bf0812c2-9323-4e53-8ad8-000613f6a4e0" />

### Next moves 

1. try AgentState enum with Idle, Task, Action. So, instead of using several With<> and Without<> on queries,
just check if AgentState holds the value the system is expecting before moving on
2. Use Required Components on Task and Action components
```
#[require(AgentState(|| AgentState::Idle))]
#[require(AgentState(|| AgentState::Task))]
#[require(AgentState(|| AgentState::Action))]

```
3. Global Resource Interrupt with some internal data HashMap<Entity, dyn X>. This dyn X is anything the Agent
was doing before starting an interaction. When the interaction finish, the Agent goes to that resource get
the Component and state he had before starting the interaction, so it can continue.


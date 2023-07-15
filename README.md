# taskly
A Todo app made in rust to keep track of all your tasks.

# Usage

```
taskly [COMMAND] [OPTIONS] [ARGS]

        OPTIONS:
            -c          Show only completed todos
            -p          Show only pending todos

        COMMANDS:
            help        Show this message
            add         Add a new todo
            edit        Edit an existing todo
            list        List all todos
            remove      Remove an existing todo
            done        Mark a todo as done
            undone      Mark a todo as undone
            clear       Remove all todos
            
        ARGS:
            The id of the todo to be edited, removed or marked as done/undone
```

    res.json({ status: "ok3" });
  });

  app.get("/todos", (_req, res) => {
    res.json(todos);
  });

  app.post("/todos", (req, res) => {
    const title = req.body?.title;
    if (typeof title !== "string" || title.trim() === "") {
      return res.status(400).json({ error: "title is required" });
    }
    const todo: Todo = { id: nextId++, title: title.trim(), done: false };
    todos.push(todo);
    res.status(201).json(todo);
  });

  app.patch("/todos/:id", (req, res) => {
    const todo = todos.find((t) => t.id === Number(req.params.id));
    if (!todo) return res.status(404).json({ error: "not found" });
    if (typeof req.body?.title === "string") todo.title = req.body.title;
    if (typeof req.body?.done === "boolean") todo.done = req.body.done;
    res.json(todo);
  });

  app.delete("/todos/:id", (req, res) => {
    const idx = todos.findIndex((t) => t.id === Number(req.params.id));
    if (idx === -1) return res.status(404).json({ error: "not found" });
    todos.splice(idx, 1);
    res.status(204).end();
  });

  return app;
}

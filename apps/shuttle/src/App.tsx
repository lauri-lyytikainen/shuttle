import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Input } from "@/components/ui/input"
import { Field, FieldDescription, FieldLabel } from "@/components/ui/field"

function App() {
  const [id, setId] = useState("");

  async function updateId() {
    setId(await invoke("node_id"));
  }

  updateId()

  return (
    <main className="bg-background">
      <div className="p-4 flex flex-col gap-8">

        <div className="">
          <h1 className="text-2xl font-bold">Shuttle</h1>
          <p> ID: {id} </p>
        </div>

        <Field>
          <FieldLabel htmlFor="file">File</FieldLabel>
          <Input type="file" id="file" />
          <FieldDescription>Select a file to share</FieldDescription>
        </Field>
      </div>
    </main>
  );
}

export default App;

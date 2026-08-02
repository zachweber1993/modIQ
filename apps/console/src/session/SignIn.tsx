import { useState } from "react";
import { Button } from "@/components/ui/button";
import { useSession } from "./SessionContext";

export function SignIn() {
  const [name, setName] = useState("");
  const { signIn } = useSession();

  return (
    <div className="flex h-screen items-center justify-center bg-background">
      <form
        className="flex w-72 flex-col gap-3"
        onSubmit={(event) => {
          event.preventDefault();
          if (name.trim()) signIn(name.trim());
        }}
      >
        <h1 className="text-lg font-medium text-foreground">modIQ</h1>
        <input
          className="h-8 rounded-lg border border-input bg-background px-2.5 text-sm outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50"
          placeholder="Your name"
          value={name}
          onChange={(event) => setName(event.target.value)}
          autoFocus
        />
        <Button type="submit" disabled={!name.trim()}>
          Sign in
        </Button>
      </form>
    </div>
  );
}

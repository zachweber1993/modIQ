import { SessionProvider, useSession } from "@/session/SessionContext";
import { SignIn } from "@/session/SignIn";
import { Console } from "@/regions/Console";

function Shell() {
  const { session } = useSession();
  return session ? <Console /> : <SignIn />;
}

function App() {
  return (
    <SessionProvider>
      <Shell />
    </SessionProvider>
  );
}

export default App;

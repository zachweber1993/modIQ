import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

/**
 * Mirrors the Rust `AssessmentSummary` DTO returned by the
 * `create_assessment` Tauri command. This is the API contract between
 * React and Rust — not a Runtime type, and not display logic.
 */
interface AssessmentSummary {
  assessmentId: string;
  evidenceCount: number;
  findingCount: number;
  recommendationCount: number;
}

function App() {
  const [summary, setSummary] = useState<AssessmentSummary | null>(null);

  async function createAssessment() {
    const response = await invoke<AssessmentSummary>("create_assessment");
    setSummary(response);
  }

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-6 bg-background text-foreground">
      <h1 className="text-3xl font-semibold">Hello, modIQ</h1>

      <Button onClick={() => void createAssessment()}>Create Assessment</Button>

      {summary && (
        <div className="flex flex-col items-center gap-1 text-muted-foreground">
          <p>Assessment ID: {summary.assessmentId}</p>
          <p>Evidence Count: {summary.evidenceCount}</p>
          <p>Finding Count: {summary.findingCount}</p>
          <p>Recommendation Count: {summary.recommendationCount}</p>
        </div>
      )}
    </main>
  );
}

export default App;

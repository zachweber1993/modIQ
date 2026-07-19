import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";

/**
 * Mirrors the Rust DTOs returned by the `create_assessment` Tauri
 * command. This is the API contract between React and Rust — not
 * Runtime types, and not display logic.
 */
interface EvidenceEntry {
  id: string;
  category: string;
  description: string;
  location: string | null;
}

interface FindingEntry {
  id: string;
  severity: string;
  description: string;
}

interface RecommendationEntry {
  id: string;
  action: string;
}

interface AssessmentSummary {
  assessmentId: string;
  evidenceCount: number;
  findingCount: number;
  recommendationCount: number;
  evidence: EvidenceEntry[];
  findings: FindingEntry[];
  recommendations: RecommendationEntry[];
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
        <div className="flex w-full max-w-md flex-col items-center gap-4 text-sm">
          <div className="flex flex-col items-center gap-1 text-muted-foreground">
            <p>Assessment ID: {summary.assessmentId}</p>
            <p>Evidence Count: {summary.evidenceCount}</p>
            <p>Finding Count: {summary.findingCount}</p>
            <p>Recommendation Count: {summary.recommendationCount}</p>
          </div>

          <div className="flex w-full flex-col gap-3">
            <div>
              <p className="font-medium">Evidence</p>
              <ul className="list-disc pl-5 text-muted-foreground">
                {summary.evidence.map((item) => (
                  <li key={item.id}>
                    {item.category}: {item.description}
                  </li>
                ))}
              </ul>
            </div>

            <div>
              <p className="font-medium">Findings</p>
              <ul className="list-disc pl-5 text-muted-foreground">
                {summary.findings.map((item) => (
                  <li key={item.id}>
                    {item.severity}: {item.description}
                  </li>
                ))}
              </ul>
            </div>

            <div>
              <p className="font-medium">Recommendations</p>
              <ul className="list-disc pl-5 text-muted-foreground">
                {summary.recommendations.map((item) => (
                  <li key={item.id}>{item.action}</li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;

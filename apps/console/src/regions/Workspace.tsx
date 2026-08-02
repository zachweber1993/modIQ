import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { Button } from "@/components/ui/button";
import { submitAssessment } from "@/engine";
import type { ReportSummary } from "@/engine";
import { Reviewing } from "@/workspace/Reviewing";

/**
 * Workspace — Intake and Reviewing only (Phase 3, Sprint 21). Which
 * of the two is active is not a separately tracked value: it is
 * derived from whether `report` holds a result. `status` and `error`
 * are transient UI state *within* Intake — not a third or fourth
 * workspace state — the pending condition carries no phase
 * information, no percentage, no Finding content, only "a request is
 * in flight."
 */
type IntakeStatus = "idle" | "submitting" | "failed";

export function Workspace() {
  const [status, setStatus] = useState<IntakeStatus>("idle");
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [report, setReport] = useState<ReportSummary | null>(null);

  const chooseFolder = async () => {
    const path = await open({ directory: true, multiple: false });
    // Cancelled: no state change, no error — fully reversible, no
    // ceremony (ASSESSMENT_INTAKE_AND_UPLOAD.md §6).
    if (!path || Array.isArray(path)) return;

    setSelectedPath(path);
    setError(null);
    setStatus("submitting");

    try {
      const result = await submitAssessment(path);
      setReport(result);
      setStatus("idle");
    } catch (err) {
      setError(String(err));
      setStatus("failed");
    }
  };

  if (report && selectedPath) {
    return <Reviewing findings={report.findings} subjectLabel={selectedPath} />;
  }

  return (
    <div className="flex h-full flex-col items-center justify-center gap-3 text-center">
      {selectedPath && (
        <p className="text-sm text-muted-foreground">Included: {selectedPath}</p>
      )}
      {status === "failed" && (
        <p className="text-sm text-destructive">
          Something went wrong: {error}
        </p>
      )}
      <Button onClick={chooseFolder} disabled={status === "submitting"}>
        {status === "submitting" ? "Submitting…" : "Choose a mod folder"}
      </Button>
    </div>
  );
}

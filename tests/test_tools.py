import hashlib
import json
import os
import shutil
import subprocess
import tempfile
import unittest

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
SYNC_SCRIPT = os.path.join(REPO_ROOT, "tools", "sync.sh")
VERIFY_SCRIPT = os.path.join(REPO_ROOT, "tools", "verify.sh")
UPDATE_HASHES_SCRIPT = os.path.join(REPO_ROOT, "tools", "update-hashes.sh")


class TestCleanBreakGovernanceTools(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.mkdtemp()

    def tearDown(self):
        shutil.rmtree(self.temp_dir, ignore_errors=True)

    def run_cmd(self, cmd, cwd=REPO_ROOT):
        res = subprocess.run(
            cmd,
            cwd=cwd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            shell=isinstance(cmd, str),
        )
        return res

    def test_sync_and_verify_default_profiles(self):
        """Test syncing spec to a new repo with default active profiles."""
        res_sync = self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        self.assertEqual(res_sync.returncode, 0, f"sync.sh failed: {res_sync.stderr}")
        self.assertIn("+ core/taxonomy.md", res_sync.stdout)
        self.assertIn("+ profiles/validation/acceptance.md", res_sync.stdout)
        self.assertIn("+ profiles/architecture/adr.md", res_sync.stdout)

        gov_dir = os.path.join(self.temp_dir, "docs", "governance", "documentation")
        self.assertTrue(os.path.isdir(gov_dir))
        self.assertTrue(os.path.exists(os.path.join(gov_dir, ".manifest.json")))
        self.assertTrue(os.path.exists(os.path.join(gov_dir, "core", "taxonomy.md")))
        self.assertTrue(os.path.exists(os.path.join(gov_dir, "core", "invariants.md")))

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertEqual(
            res_verify.returncode,
            0,
            f"verify.sh failed: {res_verify.stderr}\n{res_verify.stdout}",
        )
        self.assertIn("ok: core/taxonomy.md", res_verify.stdout)
        self.assertIn("ok: profiles/validation/acceptance.md", res_verify.stdout)
        self.assertIn("ok: profiles/architecture/adr.md", res_verify.stdout)

    def test_core_only_profile_activation(self):
        """Test activating only 'core' profile; ensures unused profiles are not copied and not flagged as drift."""
        gov_dir = os.path.join(self.temp_dir, "docs", "governance", "documentation")
        os.makedirs(gov_dir, exist_ok=True)
        contracts_file = os.path.join(gov_dir, "contracts.md")
        with open(contracts_file, "w", encoding="utf-8") as f:
            f.write(
                "# Custom Contracts\n\n"
                "## 1. Activated Profiles\n"
                "- [x] `core`\n"
                "- [ ] `validation`\n"
                "- [ ] `architecture`\n"
                "- [ ] `operations`\n"
            )

        res_sync = self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        self.assertEqual(res_sync.returncode, 0)
        self.assertIn("+ core/taxonomy.md", res_sync.stdout)
        self.assertFalse(os.path.exists(os.path.join(gov_dir, "profiles", "validation")))
        self.assertFalse(os.path.exists(os.path.join(gov_dir, "profiles", "architecture")))

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertEqual(
            res_verify.returncode,
            0,
            f"verify.sh failed on core-only repo: {res_verify.stderr}\n{res_verify.stdout}",
        )
        self.assertIn("active profiles: core", res_verify.stdout)
        self.assertIn("ok: core/taxonomy.md", res_verify.stdout)

    def test_selective_validation_profile(self):
        """Test activating 'core' and 'validation' profiles only."""
        gov_dir = os.path.join(self.temp_dir, "docs", "governance", "documentation")
        os.makedirs(gov_dir, exist_ok=True)
        contracts_file = os.path.join(gov_dir, "contracts.md")
        with open(contracts_file, "w", encoding="utf-8") as f:
            f.write(
                "# Custom Contracts\n\n"
                "## 1. Activated Profiles\n"
                "- [x] `core`\n"
                "- [x] `validation`\n"
                "- [ ] `architecture`\n"
                "- [ ] `operations`\n"
            )

        res_sync = self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        self.assertEqual(res_sync.returncode, 0)
        self.assertTrue(os.path.exists(os.path.join(gov_dir, "profiles", "validation", "acceptance.md")))
        self.assertFalse(os.path.exists(os.path.join(gov_dir, "profiles", "architecture")))

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertEqual(res_verify.returncode, 0)
        self.assertIn("active profiles: core, validation", res_verify.stdout)
        self.assertIn("ok: profiles/validation/acceptance.md", res_verify.stdout)

    def test_selective_architecture_profile(self):
        """Test activating 'core' and 'architecture' profiles only."""
        gov_dir = os.path.join(self.temp_dir, "docs", "governance", "documentation")
        os.makedirs(gov_dir, exist_ok=True)
        contracts_file = os.path.join(gov_dir, "contracts.md")
        with open(contracts_file, "w", encoding="utf-8") as f:
            f.write(
                "# Custom Contracts\n\n"
                "## 1. Activated Profiles\n"
                "- [x] `core`\n"
                "- [ ] `validation`\n"
                "- [x] `architecture`\n"
                "- [ ] `operations`\n"
            )

        res_sync = self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        self.assertEqual(res_sync.returncode, 0)
        self.assertTrue(os.path.exists(os.path.join(gov_dir, "profiles", "architecture", "adr.md")))
        self.assertTrue(os.path.exists(os.path.join(gov_dir, "profiles", "architecture", "living-snapshot.md")))
        self.assertFalse(os.path.exists(os.path.join(gov_dir, "profiles", "validation")))

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertEqual(res_verify.returncode, 0)
        self.assertIn("active profiles: architecture, core", res_verify.stdout)
        self.assertIn("ok: profiles/architecture/adr.md", res_verify.stdout)

    def test_verify_detects_drift_in_core(self):
        """Test that verify.sh detects tampering in core invariants."""
        self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        target_file = os.path.join(
            self.temp_dir, "docs", "governance", "documentation", "core", "invariants.md"
        )
        with open(target_file, "a", encoding="utf-8") as f:
            f.write("\n<!-- unauthorized tampering -->\n")

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertNotEqual(res_verify.returncode, 0)
        self.assertIn("DRIFT: core/invariants.md", res_verify.stdout)

    def test_verify_detects_drift_in_active_profile(self):
        """Test that verify.sh detects tampering in active profile files."""
        self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        target_file = os.path.join(
            self.temp_dir,
            "docs",
            "governance",
            "documentation",
            "profiles",
            "architecture",
            "adr.md",
        )
        with open(target_file, "a", encoding="utf-8") as f:
            f.write("\n<!-- profile drift -->\n")

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertNotEqual(res_verify.returncode, 0)
        self.assertIn("DRIFT: profiles/architecture/adr.md", res_verify.stdout)

    def test_verify_detects_missing_file_in_active_profile(self):
        """Test that verify.sh detects deleted files from active profiles."""
        self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        target_file = os.path.join(
            self.temp_dir,
            "docs",
            "governance",
            "documentation",
            "profiles",
            "architecture",
            "adr.md",
        )
        os.remove(target_file)

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertNotEqual(res_verify.returncode, 0)
        self.assertIn("DRIFT: missing: profiles/architecture/adr.md", res_verify.stdout)

    def test_verify_detects_extraneous_file(self):
        """Test that verify.sh detects unlisted files."""
        self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        extra_file = os.path.join(
            self.temp_dir, "docs", "governance", "documentation", "core", "intruder.md"
        )
        with open(extra_file, "w", encoding="utf-8") as f:
            f.write("# Intruder\n")

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertNotEqual(res_verify.returncode, 0)
        self.assertIn("DRIFT: not in manifest: core/intruder.md", res_verify.stdout)

    def test_sync_cleans_up_legacy_flat_files(self):
        """Test that sync.sh cleans up obsolete flat files from prior protocols."""
        gov_dir = os.path.join(self.temp_dir, "docs", "governance", "documentation")
        os.makedirs(gov_dir, exist_ok=True)
        # Create legacy flat files and v3 core files
        with open(os.path.join(gov_dir, "routing.md"), "w", encoding="utf-8") as f:
            f.write("# Old Routing\n")
        with open(os.path.join(gov_dir, "style-guide.md"), "w", encoding="utf-8") as f:
            f.write("# Old Style\n")
        os.makedirs(os.path.join(gov_dir, "core"), exist_ok=True)
        with open(os.path.join(gov_dir, "core", "routing.md"), "w", encoding="utf-8") as f:
            f.write("# Old Core Routing\n")

        res_sync = self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        self.assertEqual(res_sync.returncode, 0)
        self.assertFalse(os.path.exists(os.path.join(gov_dir, "routing.md")))
        self.assertFalse(os.path.exists(os.path.join(gov_dir, "style-guide.md")))
        self.assertFalse(os.path.exists(os.path.join(gov_dir, "core", "routing.md")))

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertEqual(res_verify.returncode, 0)

    def test_editable_contracts_retained_on_resync(self):
        """Test that local customizations in contracts.md are preserved across syncs."""
        self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        contracts_file = os.path.join(
            self.temp_dir, "docs", "governance", "documentation", "contracts.md"
        )
        custom_content = "# Project Custom Contracts\n\n## 1. Activated Profiles\n- [x] `core`\n"
        with open(contracts_file, "w", encoding="utf-8") as f:
            f.write(custom_content)

        res_resync = self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        self.assertEqual(res_resync.returncode, 0)

        with open(contracts_file, "r", encoding="utf-8") as f:
            content_after = f.read()
        self.assertEqual(content_after, custom_content)

        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, self.temp_dir])
        self.assertEqual(res_verify.returncode, 0)

    def test_force_template_overwrites_editable_contracts(self):
        """Test that sync.sh --force-template overwrites local modifications in contracts.md."""
        self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir])
        contracts_file = os.path.join(
            self.temp_dir, "docs", "governance", "documentation", "contracts.md"
        )
        custom_content = "# Project Custom Contracts\n\n## 1. Activated Profiles\n- [x] `core`\n"
        with open(contracts_file, "w", encoding="utf-8") as f:
            f.write(custom_content)

        res_force = self.run_cmd(["bash", SYNC_SCRIPT, self.temp_dir, "--force-template"])
        self.assertEqual(res_force.returncode, 0)
        self.assertIn("~ contracts.md", res_force.stdout)

        with open(contracts_file, "r", encoding="utf-8") as f:
            content_after = f.read()
        self.assertNotEqual(content_after, custom_content)

    def test_verify_fails_on_missing_manifest(self):
        """Test that verify.sh fails with a fatal error when .manifest.json is missing."""
        empty_repo = os.path.join(self.temp_dir, "empty_repo")
        os.makedirs(empty_repo, exist_ok=True)
        res_verify = self.run_cmd(["bash", VERIFY_SCRIPT, empty_repo])
        self.assertNotEqual(res_verify.returncode, 0)
        self.assertIn("FATAL: .manifest.json missing in", res_verify.stdout + res_verify.stderr)

    def test_update_hashes_recomputes_sha256(self):
        """Test that update-hashes.sh recomputes SHA-256 signatures in .manifest.json."""
        isolated_spec = os.path.join(self.temp_dir, "spec")
        isolated_tools = os.path.join(self.temp_dir, "tools")
        shutil.copytree(os.path.join(REPO_ROOT, "spec"), isolated_spec)
        shutil.copytree(os.path.join(REPO_ROOT, "tools"), isolated_tools)

        target_file = os.path.join(isolated_spec, "core", "taxonomy.md")
        with open(target_file, "a", encoding="utf-8") as f:
            f.write("\n<!-- maintainer update test -->\n")

        isolated_script = os.path.join(isolated_tools, "update-hashes.sh")
        res = self.run_cmd(["bash", isolated_script], cwd=self.temp_dir)
        self.assertEqual(res.returncode, 0, f"update-hashes.sh failed: {res.stderr}")
        self.assertIn("hashes updated", res.stdout)

        manifest_path = os.path.join(isolated_spec, ".manifest.json")
        with open(manifest_path, "r", encoding="utf-8") as f:
            m = json.load(f)

        with open(target_file, "rb") as f:
            computed_sha = hashlib.sha256(f.read()).hexdigest()
        self.assertEqual(m["files"]["core/taxonomy.md"]["sha256"], computed_sha)


if __name__ == "__main__":
    unittest.main()

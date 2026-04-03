"""CrustyClaw - Pure Rust AI assistant (extended from rustypycraw)"""

import os
import subprocess
from .security import PermissionClassifier
from .memory import MemoryLoader

class CrustyClaw:
    def __init__(self, root_path: str = "."):
        self.root_path = os.path.expanduser(root_path)
        self.security = PermissionClassifier()
        self.memory = MemoryLoader(root_path)
        self.rust_binary = "./target/release/crustyclaw"
    
    def pinch(self, path: str) -> str:
        """Run pinch mode (Rust core)"""
        if os.path.exists(self.rust_binary):
            result = subprocess.run([self.rust_binary, "pinch", path], capture_output=True, text=True)
            return result.stdout
        return "Rust binary not found. Run: cargo build --release"
    
    def ask(self, question: str) -> str:
        """Ask with security and memory context"""
        level, reason = self.security.classify(question)
        if level.name == "BLOCK":
            return f"🔒 Blocked: {reason}"
        
        context = self.memory.load_all_context(question)
        return f"🦞 CrustyClaw: {question}\n\nContext: {context[:300]}..."

def main():
    import argparse
    parser = argparse.ArgumentParser(description="CrustyClaw AI Assistant")
    parser.add_argument("--ask", "-a", help="Ask a question")
    parser.add_argument("--pinch", "-p", help="Pinch a directory")
    args = parser.parse_args()
    
    claw = CrustyClaw()
    
    if args.ask:
        print(claw.ask(args.ask))
    elif args.pinch:
        print(claw.pinch(args.pinch))
    else:
        parser.print_help()

if __name__ == "__main__":
    main()

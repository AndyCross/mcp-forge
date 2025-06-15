class McpForge < Formula
  desc "A powerful CLI tool for managing Claude Desktop MCP server configurations"
  homepage "https://github.com/AndyCross/mcp-forge"
  url "https://github.com/AndyCross/mcp-forge/archive/v0.3.0.tar.gz"
  sha256 "c633f557679c19760b99a25d5d243486defe90c076ad62ed345b8fa23c66efd9"
  license any_of: ["MIT", "Apache-2.0"]
  head "https://github.com/AndyCross/mcp-forge.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
    
    # Install shell completions
    generate_completions_from_executable(bin/"mcp-forge", "completion")
    
    # Install man page if available
    # man1.install "docs/mcp-forge.1" if File.exist?("docs/mcp-forge.1")
  end

  test do
    # Test basic functionality
    assert_match "mcp-forge", shell_output("#{bin}/mcp-forge --version")
    
    # Test help command
    assert_match "A CLI tool for managing Claude Desktop MCP server configurations", 
                 shell_output("#{bin}/mcp-forge --help")
    
    # Test template listing (should work without config)
    assert_match "filesystem", shell_output("#{bin}/mcp-forge template list")
  end
end 
package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os/exec"
)

type NodeType string

const (
	NodeTypeRoot      NodeType = "root"
	NodeTypeWorkspace NodeType = "workspace"
	NodeTypeOutput    NodeType = "output"
	NodeTypeContainer NodeType = "con"
)

type Rect struct {
	X      int `json:"x"`
	Y      int `json:"y"`
	Width  int `json:"width"`
	Height int `json:"height"`
}

type Node struct {
	ID    int      `json:"id"`
	Type  NodeType `json:"type"`
	Name  string   `json:"name"`
	Nodes []Node   `json:"nodes"`
	AppID string   `json:"app_id"`
}

type App struct {
	AppId  string
	RunCmd string
}

var apps = map[string]App{
	"alacritty": {
		AppId:  "Alacritty",
		RunCmd: "alacritty",
	},
	"firefox": {
		AppId:  "org.mozilla.firefox",
		RunCmd: "firefox",
	},
}

func printAvailableApps() {
	fmt.Println("Available apps:")
	for name := range apps {
		fmt.Println("-", name)
	}
}

func getTree() (Node, error) {
	cmd := exec.Command("swaymsg", "-t", "get_tree", "--raw")

	out, err := cmd.Output()
	if err != nil {
		return Node{}, fmt.Errorf("failed to get output from swaymsg: %w", err)
	}

	result := Node{}

	err = json.Unmarshal(out, &result)
	if err != nil {
		return Node{}, fmt.Errorf("failed to parse tree: %w", err)
	}

	return result, nil
}

func isAppRunning(tree Node, appID string) bool {
	if tree.AppID == appID {
		return true
	}
	for _, node := range tree.Nodes {
		if isAppRunning(node, appID) {
			return true
		}
	}
	return false
}

func main() {
	// Get app name from command line arguments
	flag.Parse()
	if flag.NArg() < 1 {
		printAvailableApps()
		panic("Usage: run-or-raise <app_name>")
	}
	appName := flag.Arg(0)
	// Find the app in the list
	var app *App
	for name, a := range apps {
		if name == appName {
			app = &a
			break
		}
	}

	if app == nil {
		printAvailableApps()
		panic(fmt.Sprintf("App '%s' not found\n", appName))
	}

	fmt.Println("Running or raising app:", app.RunCmd)

	tree, err := getTree()
	if err != nil {
		panic(fmt.Sprintf("Error getting tree: %v", err))
	}

	fmt.Printf("Current tree: %+v\n", tree)

	if isAppRunning(tree, app.AppId) {
		fmt.Println("App is already running, raising it.")

		cmd := exec.Command("swaymsg", "[app_id=\""+app.AppId+"\"]", "focus")
		err := cmd.Run()
		if err != nil {
			panic(fmt.Sprintf("Failed to raise app: %v", err))
		}
		fmt.Println("App raised successfully.")
	} else {
		fmt.Println("App is not running, starting it.")
		cmd := exec.Command("swaymsg", "exec", app.RunCmd)
		err := cmd.Start()
		if err != nil {
			panic(fmt.Sprintf("Failed to start app: %v", err))
		}
		fmt.Println("App started successfully.")
	}
}

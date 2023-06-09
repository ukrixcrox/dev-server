webSocket = new WebSocket("ws://127.0.0.1:8080/ws");

webSocket.onmessage = (message) => {
    console.log("Recived: ", message.data);
    if (message.data === "reload") {
        location.reload();
    }
};

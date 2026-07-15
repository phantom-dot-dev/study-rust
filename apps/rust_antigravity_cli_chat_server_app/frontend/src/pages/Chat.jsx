import React, { useState, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { Send, RefreshCw, X, MessageSquare, AlertCircle, Smile, ShieldAlert } from 'lucide-react';

export default function Chat() {
  const [socket, setSocket] = useState(null);
  const [status, setStatus] = useState('connecting'); // 'connecting', 'waiting', 'matched', 'partner_disconnected', 'disconnected'
  const [messages, setMessages] = useState([]);
  const [inputText, setInputText] = useState('');
  const messagesEndRef = useRef(null);
  const navigate = useNavigate();

  // Establish connection
  const connect = () => {
    setStatus('connecting');
    setMessages([]);
    
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    // During development, React runs on 5173 and Rust runs on 3000
    const host = window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1' 
      ? 'localhost:3000' 
      : window.location.host;
    const wsUrl = `${protocol}//${host}/ws`;

    console.log(`Connecting to WebSocket at: ${wsUrl}`);
    const ws = new WebSocket(wsUrl);

    ws.onopen = () => {
      console.log('WebSocket connection established');
    };

    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        console.log('Received message:', data);

        if (data.type === 'status') {
          setStatus(data.status);
        } else if (data.type === 'chat') {
          setMessages((prev) => [
            ...prev,
            {
              id: Math.random().toString(36).substr(2, 9),
              type: 'chat',
              sender: data.sender, // 'me' or 'stranger'
              text: data.text,
              timestamp: new Date(),
            },
          ]);
        } else if (data.type === 'system') {
          setMessages((prev) => [
            ...prev,
            {
              id: Math.random().toString(36).substr(2, 9),
              type: 'system',
              text: data.text,
              timestamp: new Date(),
            },
          ]);
        }
      } catch (err) {
        console.error('Error parsing message data:', err);
      }
    };

    ws.onclose = () => {
      console.log('WebSocket connection closed');
      setStatus('disconnected');
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      setStatus('disconnected');
    };

    setSocket(ws);
  };

  useEffect(() => {
    connect();
    return () => {
      if (socket) {
        socket.close();
      }
    };
  }, []);

  // Autoscroll
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Send message
  const handleSendMessage = (e) => {
    if (e) e.preventDefault();
    if (!inputText.trim() || status !== 'matched' || !socket) return;

    const msg = {
      type: 'send_chat',
      text: inputText.trim(),
    };

    socket.send(JSON.stringify(msg));
    setInputText('');
  };

  // Next / Find new stranger
  const handleNext = () => {
    if (status === 'disconnected') {
      connect();
      return;
    }

    if (socket && socket.readyState === WebSocket.OPEN) {
      socket.send(JSON.stringify({ type: 'next' }));
      setMessages([]);
      setStatus('waiting');
    } else {
      connect();
    }
  };

  // Stop / Disconnect
  const handleStop = () => {
    if (socket) {
      socket.close();
    }
    setStatus('disconnected');
    setMessages((prev) => [
      ...prev,
      {
        id: Math.random().toString(36).substr(2, 9),
        type: 'system',
        text: 'You disconnected from the chat.',
        timestamp: new Date(),
      },
    ]);
  };

  // Get status color and text
  const getStatusDisplay = () => {
    switch (status) {
      case 'connecting':
        return {
          text: 'Connecting to server...',
          color: 'bg-blue-500',
          textColor: 'text-blue-400',
          borderColor: 'border-blue-500/20',
          bgColor: 'bg-blue-500/10',
          pulse: true,
        };
      case 'waiting':
        return {
          text: 'Looking for a stranger...',
          color: 'bg-amber-500',
          textColor: 'text-amber-400',
          borderColor: 'border-amber-500/20',
          bgColor: 'bg-amber-500/10',
          pulse: true,
        };
      case 'matched':
        return {
          text: 'Connected with a stranger',
          color: 'bg-emerald-500',
          textColor: 'text-emerald-400',
          borderColor: 'border-emerald-500/20',
          bgColor: 'bg-emerald-500/10',
          pulse: false,
        };
      case 'partner_disconnected':
        return {
          text: 'Stranger disconnected',
          color: 'bg-rose-500',
          textColor: 'text-rose-400',
          borderColor: 'border-rose-500/20',
          bgColor: 'bg-rose-500/10',
          pulse: true,
        };
      case 'disconnected':
      default:
        return {
          text: 'Disconnected',
          color: 'bg-gray-500',
          textColor: 'text-gray-400',
          borderColor: 'border-gray-500/20',
          bgColor: 'bg-gray-500/10',
          pulse: false,
        };
    }
  };

  const statusInfo = getStatusDisplay();

  return (
    <main className="max-w-4xl mx-auto px-4 py-6 flex flex-col flex-grow w-full max-h-[calc(100vh-73px)] animate-fade-in">
      {/* Top Bar / Status Header */}
      <div 
        className={`glass-card px-6 py-4 mb-4 flex items-center justify-between border ${statusInfo.borderColor} ${statusInfo.bgColor} transition-all duration-300`}
        style={{ borderRadius: '16px' }}
      >
        <div className="flex items-center gap-3">
          <div className="relative flex h-3 w-3">
            {statusInfo.pulse && (
              <span className={`animate-ping absolute inline-flex h-full w-full rounded-full ${statusInfo.color} opacity-75`}></span>
            )}
            <span className={`relative inline-flex rounded-full h-3 w-3 ${statusInfo.color}`}></span>
          </div>
          <span className={`font-semibold tracking-wide text-sm ${statusInfo.textColor} uppercase font-display`}>
            {statusInfo.text}
          </span>
        </div>

        <div className="flex gap-2">
          {status === 'matched' && (
            <button
              onClick={handleStop}
              className="px-4 py-1.5 rounded-lg bg-rose-500/10 hover:bg-rose-500/20 border border-rose-500/30 text-rose-400 font-semibold text-xs transition-all duration-200 flex items-center gap-1.5"
            >
              <X className="w-3.5 h-3.5" />
              Disconnect
            </button>
          )}
          {(status === 'matched' || status === 'partner_disconnected' || status === 'disconnected' || status === 'waiting') && (
            <button
              onClick={handleNext}
              className="px-4 py-1.5 rounded-lg bg-violet-600 hover:bg-violet-700 text-white font-semibold text-xs transition-all duration-200 flex items-center gap-1.5 shadow-md shadow-violet-500/10"
              style={{ background: 'var(--grad-main)' }}
            >
              <RefreshCw className={`w-3.5 h-3.5 ${status === 'waiting' ? 'animate-spin' : ''}`} />
              Next Partner
            </button>
          )}
        </div>
      </div>

      {/* Main Chat Interface Box */}
      <div className="glass-card flex flex-col flex-grow overflow-hidden relative" style={{ height: '500px' }}>
        {/* Messages Scrolling Area */}
        <div className="flex-grow p-6 overflow-y-auto custom-scrollbar flex flex-col gap-4">
          {messages.length === 0 ? (
            <div className="flex-grow flex flex-col items-center justify-center text-center opacity-40 py-12">
              <MessageSquare className="w-16 h-16 mb-4 text-violet-400 animate-pulse" />
              <h3 className="font-display font-semibold text-lg text-white mb-1">
                {status === 'waiting' ? 'Finding a stranger...' : 'Waiting to connect...'}
              </h3>
              <p className="text-sm max-w-xs text-gray-400">
                {status === 'waiting' 
                  ? 'We are searching for someone available. This usually takes just a few seconds.' 
                  : 'Establish a connection to start matching with random people.'}
              </p>
            </div>
          ) : (
            messages.map((msg) => {
              if (msg.type === 'system') {
                return (
                  <div key={msg.id} className="flex justify-center my-2 animate-slide-up">
                    <span className="px-4 py-1.5 rounded-full bg-white/5 border border-white/5 text-gray-400 text-xs flex items-center gap-2">
                      <AlertCircle className="w-3.5 h-3.5 text-violet-400" />
                      {msg.text}
                    </span>
                  </div>
                );
              }

              const isMe = msg.sender === 'me';
              return (
                <div 
                  key={msg.id} 
                  className={`flex w-full ${isMe ? 'justify-end' : 'justify-start'} animate-slide-up`}
                >
                  <div className={`flex flex-col max-w-[75%] ${isMe ? 'items-end' : 'items-start'}`}>
                    {/* Sender Name */}
                    <span className="text-[10px] text-gray-500 font-bold uppercase tracking-wider mb-1 px-1">
                      {isMe ? 'You' : 'Stranger'}
                    </span>
                    {/* Bubble */}
                    <div 
                      className={`px-4 py-3 rounded-2xl text-sm leading-relaxed shadow-sm font-medium ${
                        isMe 
                          ? 'text-white' 
                          : 'bg-white/5 border border-white/10 text-white'
                      }`}
                      style={{ 
                        background: isMe ? 'var(--grad-main)' : undefined,
                        borderRadius: isMe ? '20px 20px 4px 20px' : '20px 20px 20px 4px'
                      }}
                    >
                      {msg.text}
                    </div>
                    {/* Time */}
                    <span className="text-[9px] text-gray-600 mt-1 px-1">
                      {msg.timestamp.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                    </span>
                  </div>
                </div>
              );
            })
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* Input Text Form */}
        <form 
          onSubmit={handleSendMessage}
          className="p-4 border-t border-white/5 bg-black/30 flex gap-3 items-center"
        >
          <input
            type="text"
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            disabled={status !== 'matched'}
            placeholder={
              status === 'matched' 
                ? 'Type your message...' 
                : status === 'waiting' 
                  ? 'Waiting for a stranger to connect...' 
                  : 'Start a new search to chat...'
            }
            className="flex-grow bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-violet-500/60 focus:bg-white/10 transition-all duration-200"
          />
          <button
            type="submit"
            disabled={!inputText.trim() || status !== 'matched'}
            className="p-3.5 rounded-xl bg-violet-600 text-white hover:bg-violet-700 disabled:opacity-40 disabled:hover:bg-violet-600 transition-all duration-200 shadow-md shadow-violet-500/10 flex items-center justify-center"
            style={{ background: inputText.trim() && status === 'matched' ? 'var(--grad-main)' : undefined }}
          >
            <Send className="w-4 h-4" />
          </button>
        </form>
      </div>

      {/* Safety Notice Card */}
      <div className="flex items-center gap-2 mt-4 px-4 opacity-50 justify-center text-center">
        <ShieldAlert className="w-4 h-4 text-pink-500" />
        <span className="text-[11px] text-gray-400">
          Be respectful. Do not share sensitive personal information (address, bank details, credentials) with strangers.
        </span>
      </div>
    </main>
  );
}

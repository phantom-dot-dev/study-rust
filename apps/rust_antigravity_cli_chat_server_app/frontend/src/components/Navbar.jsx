import React from 'react';
import { Link } from 'react-router-dom';
import { Sparkles, HelpCircle, MessageSquare } from 'lucide-react';

export default function Navbar() {
  return (
    <header className="w-full border-b border-white/5 bg-black/20 backdrop-blur-md sticky top-0 z-50">
      <div className="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
        <Link to="/" className="flex items-center gap-3 group text-decoration-none">
          <div className="p-2 bg-gradient-to-tr from-violet-600 to-pink-500 rounded-xl shadow-lg shadow-violet-500/20 group-hover:scale-105 transition-transform duration-300 flex items-center justify-center" style={{ background: 'var(--grad-main)' }}>
            <MessageSquare className="w-6 h-6 text-white" />
          </div>
          <span className="font-semibold text-xl tracking-tight text-white font-display">
            Antigravity<span className="text-gradient">Chat</span>
          </span>
        </Link>

        <div className="flex items-center gap-6">
          <Link 
            to="/" 
            className="text-sm font-medium text-gray-400 hover:text-white transition-colors duration-200"
            style={{ textDecoration: 'none' }}
          >
            Home
          </Link>
          <Link 
            to="/chat" 
            className="text-sm font-semibold px-4 py-2 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 text-white transition-all duration-200 shadow-sm"
            style={{ textDecoration: 'none' }}
          >
            Start Chat
          </Link>
        </div>
      </div>
    </header>
  );
}

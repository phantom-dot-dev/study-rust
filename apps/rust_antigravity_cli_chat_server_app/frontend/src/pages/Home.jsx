import React from 'react';
import { Link } from 'react-router-dom';
import { Shield, Zap, Sparkles, MessageSquare } from 'lucide-react';

export default function Home() {
  return (
    <main className="max-w-6xl mx-auto px-6 py-12 md:py-20 flex flex-col items-center justify-center flex-grow text-center animate-fade-in">
      {/* Badge */}
      <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-violet-500/10 border border-violet-500/20 text-violet-400 text-sm font-semibold mb-6">
        <Sparkles className="w-4 h-4" />
        <span>Instantly Match & Chat</span>
      </div>

      {/* Main Heading */}
      <h1 className="font-display font-extrabold text-5xl md:text-7xl leading-tight max-w-3xl mb-6">
        Meet New People. <br />
        <span className="text-gradient">No Strings Attached.</span>
      </h1>

      {/* Slogan */}
      <p className="text-gray-400 text-lg md:text-xl max-w-2xl mb-12 font-light leading-relaxed">
        Connect instantly with random available users around the world. No registration, no logs, no trackers. Just open the page and start talking.
      </p>

      {/* Call to Action */}
      <div className="flex flex-col sm:flex-row gap-4 mb-20">
        <Link
          to="/chat"
          className="px-8 py-4 rounded-xl font-bold text-white shadow-xl shadow-violet-500/20 hover:shadow-violet-500/35 transition-all duration-300 transform hover:-translate-y-0.5 active:translate-y-0 flex items-center justify-center gap-2 text-lg"
          style={{ 
            background: 'var(--grad-main)', 
            textDecoration: 'none' 
          }}
        >
          <MessageSquare className="w-5 h-5" />
          Start Chatting Now
        </Link>
      </div>

      {/* Features Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-8 w-full max-w-5xl">
        {/* Feature 1 */}
        <div className="glass-card p-8 flex flex-col items-center text-center transform hover:-translate-y-1.5 transition-all duration-300 hover:bg-[rgba(28,33,53,0.85)]">
          <div className="p-3 bg-violet-600/10 border border-violet-600/20 rounded-2xl mb-5 text-violet-400">
            <Shield className="w-8 h-8" />
          </div>
          <h3 className="font-display font-bold text-xl text-white mb-2">Completely Anonymous</h3>
          <p className="text-gray-400 text-sm leading-relaxed">
            We don't ask for your name, email, or any personal details. You are just a stranger to everyone.
          </p>
        </div>

        {/* Feature 2 */}
        <div className="glass-card p-8 flex flex-col items-center text-center transform hover:-translate-y-1.5 transition-all duration-300 hover:bg-[rgba(28,33,53,0.85)]">
          <div className="p-3 bg-pink-600/10 border border-pink-600/20 rounded-2xl mb-5 text-pink-400">
            <Zap className="w-8 h-8" />
          </div>
          <h3 className="font-display font-bold text-xl text-white mb-2">Lightning Fast</h3>
          <p className="text-gray-400 text-sm leading-relaxed">
            Powered by a Rust WebSocket server, messages are sent and received with sub-millisecond latency.
          </p>
        </div>

        {/* Feature 3 */}
        <div className="glass-card p-8 flex flex-col items-center text-center transform hover:-translate-y-1.5 transition-all duration-300 hover:bg-[rgba(28,33,53,0.85)]">
          <div className="p-3 bg-blue-600/10 border border-blue-600/20 rounded-2xl mb-5 text-blue-400">
            <Sparkles className="w-8 h-8" />
          </div>
          <h3 className="font-display font-bold text-xl text-white mb-2">Smart Matchmaking</h3>
          <p className="text-gray-400 text-sm leading-relaxed">
            Our queue system matches you with the first available stranger. Finished talking? Just click next.
          </p>
        </div>
      </div>
    </main>
  );
}

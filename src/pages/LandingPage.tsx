import React from "react";
import { useNavigate } from "react-router-dom";
import {
  Sparkles,
  Shield,
  TrendingUp,
  Lock,
  ArrowRight,
  Zap,
  DollarSign,
} from "lucide-react";

const LandingPage: React.FC = () => {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen relative overflow-hidden">
      {/* Animated Background Blobs */}
      <div className="fixed inset-0 overflow-hidden pointer-events-none mx-auto">
        <div className="absolute top-0 left-1/4 w-96 h-96 bg-indigo-500/20 rounded-full blur-3xl animate-float"></div>
        <div
          className="absolute top-1/4 right-1/4 w-96 h-96 bg-purple-500/20 rounded-full blur-3xl animate-float"
          style={{ animationDelay: "2s" }}
        ></div>
        <div
          className="absolute bottom-1/4 left-1/3 w-96 h-96 bg-cyan-500/20 rounded-full blur-3xl animate-float"
          style={{ animationDelay: "4s" }}
        ></div>
      </div>

      {/* Hero Section */}
      <header className="relative pt-32 pb-20 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto text-center">
          {/* Badge */}
          <div className="inline-flex items-center gap-2 px-6 py-3 mb-8 glass rounded-full border border-indigo-500/30 dark:border-indigo-500/30 shadow-lg shadow-indigo-500/20 dark:shadow-indigo-500/20 backdrop-blur-xl">
            <Sparkles className="w-5 h-5 text-indigo-600 dark:text-indigo-400" />
            <span className="text-sm font-semibold text-gradient-cyber">
              Powered by Stellar Blockchain
            </span>
          </div>

          {/* Main Headline */}
          <h1 className="text-7xl md:text-8xl lg:text-9xl font-bold mb-8 leading-none">
            <span className="block text-gray-900 dark:text-white mb-2">
              Turn
            </span>
            <span className="block text-gradient-cyber">Remittance</span>
            <span className="block text-gray-900 dark:text-white">
              Into Capital
            </span>
          </h1>

          {/* Subheadline */}
          <p className="text-xl md:text-2xl text-gray-700 dark:text-white mb-6 max-w-3xl mx-auto leading-relaxed">
            The first decentralized lending platform where your remittance
            history becomes your credit score
          </p>

          <p className="text-lg text-cyan-600 dark:text-cyan-400 font-medium mb-12">
            No traditional credit check • Instant verification • Automatic
            repayments
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
            <button
              type="button"
              onClick={() => {
                void navigate("/verify");
              }}
              className="group relative px-10 py-5 bg-linear-to-r from-indigo-600 via-purple-600 to-cyan-600 rounded-2xl font-bold text-white text-lg shadow-2xl shadow-indigo-500/50 hover:shadow-indigo-500/80 transform hover:scale-105 transition-all duration-300 overflow-hidden"
            >
              <span className="relative z-10 flex items-center gap-2">
                Get Started Free
                <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
              </span>
              <div className="absolute inset-0 bg-linear-to-r from-indigo-500 via-purple-500 to-cyan-500 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            </button>

            <button
              type="button"
              onClick={() => {
                void navigate("/borrow");
              }}
              className="px-10 py-5 glass rounded-2xl font-bold text-gray-900 dark:text-white text-lg border border-gray-300 dark:border-white/20 hover:border-gray-400 dark:hover:border-white/40 hover:bg-gray-100 dark:hover:bg-white/10 transform hover:scale-105 transition-all duration-300"
            >
              View Dashboard
            </button>
          </div>

          {/* Stats */}
          <div className="grid grid-cols-3 gap-8 mt-20 max-w-3xl mx-auto">
            {[
              { value: "$10M+", label: "Total Volume" },
              { value: "5,000+", label: "Active Users" },
              { value: "98%", label: "Success Rate" },
            ].map((stat, index) => (
              <div
                key={index}
                className="glass rounded-2xl p-6 border border-gray-300 dark:border-white/10 backdrop-blur-xl card-shine"
              >
                <div className="text-4xl font-bold text-gradient-cyber mb-2">
                  {stat.value}
                </div>
                <div className="text-sm text-gray-600 dark:text-white">
                  {stat.label}
                </div>
              </div>
            ))}
          </div>
        </div>
      </header>

      {/* Features Section */}
      <main className="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
        {/* For Borrowers & Lenders */}
        <div className="grid md:grid-cols-2 gap-8 mb-32">
          {/* Borrowers Card */}
          <div className="group relative card-shine">
            <div className="glass rounded-3xl p-10 border border-gray-300 dark:border-white/10 hover:border-indigo-500/50 dark:hover:border-indigo-500/50 transition-all duration-500 backdrop-blur-xl h-full">
              {/* Gradient Orb */}
              <div className="absolute top-8 right-8 w-32 h-32 bg-linear-to-br from-indigo-500 to-purple-600 rounded-full blur-3xl opacity-20 group-hover:opacity-40 transition-opacity"></div>

              <div className="relative z-10">
                <div className="w-20 h-20 bg-linear-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center mb-8 shadow-lg shadow-indigo-500/50 group-hover:scale-110 group-hover:rotate-3 transition-all duration-500">
                  <DollarSign className="w-10 h-10 text-white" />
                </div>

                <h2 className="text-4xl font-bold text-gray-900 dark:text-white mb-4">
                  For Borrowers
                </h2>
                <p className="text-gray-700 dark:text-white text-lg mb-8 leading-relaxed">
                  Transform your remittance history into instant collateral. No
                  credit checks, no paperwork.
                </p>

                <ul className="space-y-4 mb-10">
                  {[
                    "NFT-based collateral from remittance history",
                    "Interest rates as low as 4.5% APR",
                    "Automatic loan repayments",
                    "Instant approval with verified history",
                  ].map((feature, index) => (
                    <li
                      key={index}
                      className="flex items-start gap-3 text-gray-700 dark:text-white"
                    >
                      <div className="w-6 h-6 bg-indigo-500/20 rounded-lg flex items-center justify-center shrink-0 mt-0.5">
                        <svg
                          className="w-4 h-4 text-indigo-400"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M5 13l4 4L19 7"
                          />
                        </svg>
                      </div>
                      <span>{feature}</span>
                    </li>
                  ))}
                </ul>

                <button
                  type="button"
                  onClick={() => {
                    void navigate("/verify");
                  }}
                  className="w-full bg-linear-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-bold py-4 px-6 rounded-xl transition-all duration-300 shadow-lg shadow-indigo-500/30 hover:shadow-indigo-500/50 transform hover:scale-105 flex items-center justify-center gap-2"
                >
                  Start Borrowing
                  <ArrowRight className="w-5 h-5" />
                </button>
              </div>
            </div>
          </div>

          {/* Lenders Card */}
          <div className="group relative card-shine">
            <div className="glass rounded-3xl p-10 border border-gray-300 dark:border-white/10 hover:border-cyan-500/50 dark:hover:border-cyan-500/50 transition-all duration-500 backdrop-blur-xl h-full">
              {/* Gradient Orb */}
              <div className="absolute top-8 right-8 w-32 h-32 bg-linear-to-br from-cyan-500 to-emerald-600 rounded-full blur-3xl opacity-20 group-hover:opacity-40 transition-opacity"></div>

              <div className="relative z-10">
                <div className="w-20 h-20 bg-linear-to-br from-cyan-500 to-emerald-600 rounded-2xl flex items-center justify-center mb-8 shadow-lg shadow-cyan-500/50 group-hover:scale-110 group-hover:rotate-3 transition-all duration-500">
                  <TrendingUp className="w-10 h-10 text-white" />
                </div>

                <h2 className="text-4xl font-bold text-gray-900 dark:text-white mb-4">
                  For Lenders
                </h2>
                <p className="text-gray-700 dark:text-white text-lg mb-8 leading-relaxed">
                  Earn competitive yields by providing liquidity to verified
                  borrowers.
                </p>

                <ul className="space-y-4 mb-10">
                  {[
                    "Up to 12.5% APY on deposits",
                    "Real-time pool analytics",
                    "Withdraw anytime with liquidity",
                    "Automated diversification",
                  ].map((feature, index) => (
                    <li
                      key={index}
                      className="flex items-start gap-3 text-gray-700 dark:text-white"
                    >
                      <div className="w-6 h-6 bg-cyan-500/20 rounded-lg flex items-center justify-center shrink-0 mt-0.5">
                        <svg
                          className="w-4 h-4 text-cyan-400"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M5 13l4 4L19 7"
                          />
                        </svg>
                      </div>
                      <span>{feature}</span>
                    </li>
                  ))}
                </ul>

                <button
                  type="button"
                  onClick={() => {
                    void navigate("/lend");
                  }}
                  className="w-full bg-linear-to-r from-cyan-600 to-emerald-600 hover:from-cyan-500 hover:to-emerald-500 text-white font-bold py-4 px-6 rounded-xl transition-all duration-300 shadow-lg shadow-cyan-500/30 hover:shadow-cyan-500/50 transform hover:scale-105 flex items-center justify-center gap-2"
                >
                  Start Lending
                  <ArrowRight className="w-5 h-5" />
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* How It Works */}
        <section className="mb-32">
          <div className="text-center mb-16">
            <h2 className="text-5xl md:text-6xl font-bold text-gray-900 dark:text-white mb-6">
              How It <span className="text-gradient-cyber">Works</span>
            </h2>
            <p className="text-2xl text-gray-700 dark:text-white">
              Four simple steps to financial freedom
            </p>
          </div>

          <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
            {[
              {
                number: "01",
                title: "Verify History",
                description:
                  "Connect your remittance provider to verify your payment history securely.",
                icon: Shield,
                color: "from-blue-500 to-blue-600",
                glow: "shadow-blue-500/50",
              },
              {
                number: "02",
                title: "Mint NFT",
                description:
                  "Receive your Remittance NFT with a reliability score as collateral.",
                icon: Zap,
                color: "from-purple-500 to-purple-600",
                glow: "shadow-purple-500/50",
              },
              {
                number: "03",
                title: "Get Funded",
                description:
                  "Apply for loans with competitive rates based on your NFT score.",
                icon: TrendingUp,
                color: "from-emerald-500 to-emerald-600",
                glow: "shadow-emerald-500/50",
              },
              {
                number: "04",
                title: "Auto-Repay",
                description:
                  "Automated repayments from your remittance flow—set it and forget it.",
                icon: Lock,
                color: "from-orange-500 to-orange-600",
                glow: "shadow-orange-500/50",
              },
            ].map((step, index) => {
              const Icon = step.icon;
              return (
                <div
                  key={index}
                  className="group glass rounded-3xl p-8 border border-gray-300 dark:border-white/10 hover:border-gray-400 dark:hover:border-white/20 transition-all duration-500 backdrop-blur-xl card-shine hover:-translate-y-2"
                >
                  <div
                    className={`w-16 h-16 bg-linear-to-br ${step.color} rounded-2xl flex items-center justify-center mb-6 shadow-lg ${step.glow} group-hover:scale-110 group-hover:rotate-6 transition-all duration-500`}
                  >
                    <Icon className="w-8 h-8 text-white" />
                  </div>

                  <div
                    className={`text-5xl font-bold bg-linear-to-br ${step.color} bg-clip-text text-transparent mb-4`}
                  >
                    {step.number}
                  </div>

                  <h3 className="text-2xl font-bold text-gray-900 dark:text-white mb-3">
                    {step.title}
                  </h3>
                  <p className="text-gray-700 dark:text-white leading-relaxed">
                    {step.description}
                  </p>
                </div>
              );
            })}
          </div>
        </section>

        {/* Final CTA */}
        <section className="relative glass rounded-3xl p-16 text-center border border-gray-300 dark:border-white/10 backdrop-blur-xl overflow-hidden">
          <div className="absolute inset-0 bg-linear-to-r from-indigo-600/20 via-purple-600/20 to-cyan-600/20 animate-gradient"></div>

          <div className="relative z-10">
            <h2 className="text-5xl md:text-6xl font-bold text-gray-900 dark:text-white mb-6">
              Ready to <span className="text-gradient-cyber">Transform</span>{" "}
              Your Future?
            </h2>
            <p className="text-xl md:text-2xl text-gray-700 dark:text-white mb-10 max-w-2xl mx-auto">
              Join thousands of users leveraging their remittance history for
              instant capital
            </p>
            <button
              type="button"
              onClick={() => {
                void navigate("/verify");
              }}
              className="px-12 py-6 bg-linear-to-r from-indigo-600 via-purple-600 to-cyan-600 rounded-2xl font-bold text-white text-xl shadow-2xl shadow-indigo-500/50 hover:shadow-indigo-500/80 transform hover:scale-105 transition-all duration-300 inline-flex items-center gap-3"
            >
              Start Verification Now
              <ArrowRight className="w-6 h-6" />
            </button>
          </div>
        </section>
      </main>

      {/* Footer */}
      <footer className="relative text-center py-12 px-4 mt-20">
        <div className="glass rounded-2xl p-8 max-w-4xl mx-auto border border-gray-300 dark:border-white/10 backdrop-blur-xl">
          <p className="text-gray-700 dark:text-white">
            © 2025 RemitLend. All rights reserved. Built on Stellar Blockchain.
          </p>
          <div className="flex justify-center gap-6 mt-4">
            <a
              href="#"
              className="text-gray-600 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
            >
              Privacy
            </a>
            <a
              href="#"
              className="text-gray-600 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
            >
              Terms
            </a>
            <a
              href="#"
              className="text-gray-600 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors"
            >
              Docs
            </a>
          </div>
        </div>
      </footer>
    </div>
  );
};

export default LandingPage;

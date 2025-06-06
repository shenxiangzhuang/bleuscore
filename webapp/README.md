# BleuScore Online - Fast BLEU Score Calculator

A modern web application for calculating BLEU scores using WebAssembly-powered computation.

## 🚀 Live Demo

Visit the live application: **[https://shenxiangzhuang.github.io/bleuscore/](https://shenxiangzhuang.github.io/bleuscore/)**

## ✨ Features

- **Fast Computation**: Powered by Rust WebAssembly for optimal performance
- **Modern UI**: Clean, responsive design with Tailwind CSS
- **Real-time Calculation**: Instant BLEU score computation
- **Multiple N-grams**: Support for 1-gram to 5-gram precision
- **Smoothing Options**: Optional smoothing for better scores
- **Detailed Metrics**: Complete breakdown including brevity penalty and precision bars

## 🛠️ Development

### Prerequisites

- Node.js 18 or higher
- npm

### Local Development

```bash
# Install dependencies
npm install

# Start development server
npm start

# Build for production
npm run build
```

The development server will start at `http://localhost:8080` with hot reload enabled.

### Project Structure

```
webapp/
├── app.js          # Main application logic
├── index.html      # HTML template
├── package.json    # Dependencies and scripts
├── webpack.config.js # Webpack configuration
└── dist/           # Built files (generated)
```

## 📦 Deployment

### GitHub Pages (Automatic)

The application automatically deploys to GitHub Pages when changes are pushed to the main branch:

1. **Push to main branch** - triggers GitHub Actions workflow
2. **Build process** - webpack builds the production bundle
3. **Deploy** - files are automatically deployed to GitHub Pages

### Manual Deployment

```bash
# Build for production
npm run build

# The dist/ folder contains all files needed for deployment
```

## 🧪 Usage

1. **Enter Predictions**: Add your machine translation outputs (one per line)
2. **Enter References**: Add reference translations (one per line)
3. **Configure Settings**: 
   - Choose N-gram order (1-5)
   - Enable/disable smoothing
4. **Calculate**: Click "Calculate BLEU Score" or press Ctrl+Enter
5. **View Results**: See detailed scores, metrics, and precision breakdown

## 🔧 Configuration

### Webpack

The webpack configuration supports both development and production modes:

- **Development**: Hot reload, source maps, unminified output
- **Production**: Minified output, content hashing, optimized chunks

### Environment Variables

- Production builds automatically set `publicPath` to `/bleuscore/` for GitHub Pages
- Development uses root path `/` for local development

## 📊 BLEU Score Interpretation

| Score Range | Quality Level |
|-------------|---------------|
| 0.8 - 1.0   | Excellent     |
| 0.6 - 0.8   | Good          |
| 0.4 - 0.6   | Fair          |
| 0.0 - 0.4   | Poor          |

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally with `npm start`
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- **BleuScore**: Built with the [bleuscore-js](https://www.npmjs.com/package/bleuscore-js) WebAssembly package
- **UI Framework**: [Tailwind CSS](https://tailwindcss.com/)
- **Bundler**: [Webpack](https://webpack.js.org/)
- **Deployment**: [GitHub Pages](https://pages.github.com/)

## 📊 What is BLEU Score?

BLEU (Bilingual Evaluation Understudy) is a metric for automatically evaluating machine-translated text. It compares candidate translations with reference translations using n-gram precision and brevity penalty.

### Score Interpretation
- **0.8-1.0**: Excellent quality
- **0.6-0.8**: Good quality  
- **0.4-0.6**: Fair quality
- **0.0-0.4**: Poor quality

## 🛠️ Local Development

### Prerequisites
- Python 3.x (for local server)
- Modern web browser

### Running Locally

1. Clone the repository:
```bash
git clone https://github.com/shenxiangzhuang/bleuscore.git
cd bleuscore/webapp
```

2. Start a local server:
```bash
npm run serve
# or
python3 -m http.server 8080
```

3. Open your browser and navigate to `http://localhost:8080`

## 📦 Using bleuscore-js Package

This application uses the `bleuscore-js` npm package. You can also use it in your own projects:

### Installation
```bash
npm install bleuscore-js
```

### Usage
```javascript
import BleuScore from 'bleuscore-js';

const reference = "The cat is sitting on the mat.";
const prediction = "The cat is on the mat.";
const maxN = 4;

const result = BleuScore.compute(reference, prediction, maxN);
console.log('BLEU Score:', result.bleu);
console.log('Precisions:', result.precisions);
console.log('Brevity Penalty:', result.brevity_penalty);
```

## 🌐 GitHub Pages Deployment

This application is designed to work seamlessly with GitHub Pages:

1. **Fork the repository** or create your own
2. **Enable GitHub Pages** in repository settings
3. **Set source** to the `webapp` directory or copy files to root
4. **Access your app** at `https://yourusername.github.io/repositoryname/`

### Automatic Deployment
The application loads `bleuscore-js` directly from CDN, making it completely static and GitHub Pages compatible.

## 🎯 Usage Instructions

1. **Enter Predictions**: Add your machine translation outputs, one per line
2. **Enter References**: Add reference translations, one per line
3. **Configure Settings**: Choose n-gram order (1-5) and enable smoothing if needed
4. **Calculate**: Click "Calculate BLEU Score" or press `Ctrl+Enter`
5. **View Results**: See detailed BLEU metrics with visual representations

## 🔧 Technical Details

- **Frontend**: Vanilla JavaScript, HTML5, CSS3
- **Styling**: Tailwind CSS via CDN
- **BLEU Engine**: bleuscore-js npm package
- **Deployment**: Static files, GitHub Pages compatible
- **Browser Support**: All modern browsers

## 📝 Input Format

### Predictions
```
The cat is on the mat.
It is a good day.
Machine translation works well.
```

### References  
```
The cat is sitting on the mat.
Today is a beautiful day.
Machine translation performs well.
```

## 🔗 Related Links

- [BleuScore GitHub Repository](https://github.com/shenxiangzhuang/bleuscore)
- [BLEU Score Wikipedia](https://en.wikipedia.org/wiki/BLEU)
- [bleuscore-js npm package](https://www.npmjs.com/package/bleuscore-js) 